//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1108/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1108<F: Float>(t6824: F, t9270: F, t328: F, t6045: F, t824: F, t822: F, t833: F, t2397: F, t6745: F, t2242: F, t2355: F, t6810: F, t8801: F) -> (F, F, F, F, F) {
    let t19836 = t9270 * t6824;
    let t19839 = t824 * t328 * t6045;
    let t19841 = t822 * t19839 * t833;
    let t19843 = t6745 * t2397;
    let t19845 = t2242 * t2355;
    let t19857 = t8801 * t6810;
    (t19836, t19841, t19843, t19845, t19857)
}
