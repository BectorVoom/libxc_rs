//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 455/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk455<F: Float>(t1033: F, t636: F, t1045: F, t582: F, t211: F, t1023: F, t616: F, t1018: F, t185: F, t1001: F, t395: F, t1014: F, t401: F, t1004: F, t172: F, t184: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t2747 = t1033 * t636;
    let t2749 = t582 * t1045;
    let t2750 = t211 * t2749;
    let t2753 = t582 * t1023;
    let t2754 = t616 * t2753;
    let t2756 = t582 * t1018;
    let t2757 = t185 * t2756;
    let t2760 = t395 * t1001;
    let t2773 = t401 * t1014;
    let t2789 = t172 * t1004;
    let t2790 = t2789 * t184;
    (t2747, t2749, t2750, t2753, t2754, t2756, t2757, t2760, t2773, t2789, t2790)
}
