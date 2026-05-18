//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1188/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1188<F: Float>(t2643: F, t7269: F, t2699: F, t7520: F, t7516: F, t1110: F, t2635: F, t7410: F, t7515: F, t2636: F, t2674: F, t7237: F, t7241: F) -> (F, F, F, F, F) {
    let t22010 = t2643 * t7269;
    let t22012 = t2699 * t7520;
    let t22014 = t2643 * t7516;
    let t22019 = F::new(0.69263436422725855036e2) * t1110 * t2635 * t7410 * t7515;
    let t22024 = F::new(0.61524113149298439947e4) * t1110 * t7237 * t2674 * t7241 * t2636;
    (t22010, t22012, t22014, t22019, t22024)
}
