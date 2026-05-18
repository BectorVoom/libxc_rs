//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 885/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk885<F: Float>(t2635: F, t2674: F, t7515: F, t1110: F, t16: F, t3021: F, t492: F, t1105: F, t2699: F, t2707: F, t1052: F, t2742: F) -> (F, F, F, F, F, F) {
    let t7516 = t2635 * t2674 * t7515;
    let t7518 = F::new(0.51947577317044391277e2) * t1110 * t7516;
    let t7520 = t16 * t3021 * t492;
    let t7522 = F::new(0.56968947174242584612e-3) * t1105 * t7520;
    let t7523 = t2699 * t2707;
    let t7526 = t1052 * t2742;
    (t7516, t7518, t7520, t7522, t7523, t7526)
}
