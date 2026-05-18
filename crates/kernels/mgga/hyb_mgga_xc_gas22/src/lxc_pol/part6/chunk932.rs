//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 932/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk932<F: Float>(t139: F, t8438: F, t214: F, t26: F, t2950: F, t765: F, t1240: F, t2018: F, t3279: F, t677: F, t1319: F, t1815: F) -> (F, F, F, F, F, F, F) {
    let t8439 = t139 * t8438;
    let t8440 = t8439 * t214;
    let t8441 = t26 * t8440;
    let t8446 = t2950 * t765;
    let t8450 = t1240 * t2018 / F::new(32.0);
    let t8452 = t677 * t3279 / F::new(32.0);
    let t8453 = t1815 * t1319;
    (t8439, t8440, t8441, t8446, t8450, t8452, t8453)
}
