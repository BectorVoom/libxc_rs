//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1208/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1208(t96261: f64, t96270: f64, t28171: f64, t7784: f64, t26966: f64, t28160: f64, t27023: f64, t28190: f64, t27006: f64, t96339: f64, t96345: f64, t26960: f64, t96975: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t97352 = 0.61905925925925925925e-2_f64 * t96261;
    let t97360 = 0.61905925925925925925e-2_f64 * t96270;
    let t97377 = 0.61782407407407407408e-3_f64 * t28171 * t7784;
    let t97385 = 0.61782407407407407408e-3_f64 * t26966 * t28160;
    let t97387 = 0.23168402777777777778e-3_f64 * t28190 * t27023;
    let t97407 = 0.7722800925925925926e-4_f64 * t28190 * t27006;
    let t97420 = 0.10317654320987654321e-2_f64 * t96339;
    let t97422 = 0.30952962962962962962e-2_f64 * t96345;
    let t97428 = 0.7722800925925925926e-4_f64 * t26960 * t96975;
    (t97352, t97360, t97377, t97385, t97387, t97407, t97420, t97422, t97428)
}
