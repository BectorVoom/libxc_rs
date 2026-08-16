//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1373/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1373(t28171: f64, t7784: f64, t26966: f64, t28160: f64, t27023: f64, t28190: f64, t27003: f64, t330: f64, t5314: f64, t7788: f64, t7789: f64, t8095: f64, t92613: f64, t93158: f64, t93161: f64, t96273: f64, t96298: f64, t96311: f64, t97039: f64) -> f64 {
    let t97377 = 0.61782407407407407408e-3_f64 * t28171 * t7784;
    let t97385 = 0.61782407407407407408e-3_f64 * t26966 * t28160;
    let t97387 = 0.23168402777777777778e-3_f64 * t28190 * t27023;
    let t97393 = -0.15445601851851851852e-3_f64 * t28190 * t27003 - 0.38691203703703703703e-3_f64 * t96273 + t93158 + t97377 - 0.23168402777777777778e-3_f64 * t7788 * t5314 * t7789 * t330 + 0.33980324074074074074e-2_f64 * t92613 * t8095 - t97385 + t97387 + 0.11349419753086419753e-1_f64 * t96298 - 0.30945286961263020833e-5_f64 * t93161 - 0.13901041666666666667e-2_f64 * t7788 * t97039 + 0.38691203703703703704e-2_f64 * t96311;
    t97393
}
