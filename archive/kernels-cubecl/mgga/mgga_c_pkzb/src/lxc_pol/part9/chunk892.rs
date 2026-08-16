//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 892/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk892<F: Float>(t6012: F, t6517: F, t6556: F, t2363: F, t937: F, t410: F, t919: F, t2970: F, t6417: F, t6523: F, t2370: F, t2421: F, t914: F) -> (F, F, F, F, F, F, F) {
    let t6557 = t6012 * t6517;
    let t6558 = t6556 * t6557;
    let t6561 = t2363 * t937;
    let t6565 = t2363 * t410 * t919;
    let t6566 = t2970 * t6417;
    let t6569 = t6523 * t410;
    let t6570 = t6012 * t2370;
    let t6571 = t6556 * t6570;
    let t6574 = t914 * t2421;
    (t6558, t6561, t6565, t6566, t6569, t6571, t6574)
}
