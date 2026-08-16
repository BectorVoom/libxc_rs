//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1308/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1308(t1616: f64, t7492: f64, t29280: f64, t4142: f64, t102294: f64, t7978: f64, t102029: f64, t102088: f64, t102137: f64, t102371: f64, t1307: f64, t18187: f64, t21655: f64, t27567: f64, t27583: f64, t28721: f64, t28738: f64, t28765: f64, t28807: f64, t4440: f64, t7968: f64, t99331: f64, t99424: f64, t99437: f64) -> (f64, f64) {
    let t102412 = t1616 * t7492;
    let t102421 = t4142 * t29280;
    let t102425 = t7978 * t102294;
    let t102427 = 0.11584201388888888889e-3_f64 * t27583 * t102088 - 0.61782407407407407407e-3_f64 * t99331 * t28807 - 0.61836467013888888888e-4_f64 * t27567 * t102137 + t99424 + 0.61782407407407407408e-3_f64 * t27583 * t18187 * t28765 * t21655 + t99437 + 0.11584201388888888889e-3_f64 * t27583 * t4440 * t102412 * t1307 - 0.92754700520833333334e-4_f64 * t7968 * t102029 - 0.92754700520833333334e-4_f64 * t7968 * t102371 + 0.10317654320987654321e-2_f64 * t102421 - 0.92754700520833333334e-4_f64 * t28721 * t28738 - 0.23168402777777777778e-3_f64 * t102425;
    (t102421, t102427)
}
