//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 870/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk870(t610: f64, t7468: f64, t7467: f64, t1820: f64, t1033: f64, t1683: f64, t2816: f64, t663: f64, t2749: f64, t633: f64, t5338: f64, t5347: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7469 = t7468 * t610;
    let t7470 = t7467 * t7469;
    let t7472 = 8.0_f64 / 15.0_f64 * t1820 * t7470;
    let t7474 = 8.0_f64 / 45.0_f64 * t1033 * t1683;
    let t7476 = 4.0_f64 / 15.0_f64 * t2816 * t663;
    let t7478 = 8.0_f64 / 45.0_f64 * t633 * t2749;
    let t7479 = 16.0_f64 / 45.0_f64 * t5338;
    let t7480 = 8.0_f64 / 45.0_f64 * t5347;
    (t7472, t7474, t7476, t7478, t7479, t7480)
}
