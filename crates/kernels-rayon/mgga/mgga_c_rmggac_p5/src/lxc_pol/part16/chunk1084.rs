//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1084/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1084(t10417: f64, t39535: f64, t39555: f64, t4041: f64, t43001: f64, t43008: f64, t45601: f64, t45603: f64, t45608: f64, t45610: f64, t45614: f64, t45617: f64, t45630: f64, t45633: f64, t45636: f64, t45641: f64, t45646: f64, t5928: f64, t9427: f64) -> f64 {
    let t48564 = -t43001 + 0.47896966807455234255e0_f64 * t39535 + t43008 - 0.212822999466489197e-4_f64 * t45601 - 0.1064114997332445985e-4_f64 * t45603 - 0.20496175532535769483e-3_f64 * t39555 + 0.85129199786595678799e-5_f64 * t45608 + 0.1702583995731913576e-4_f64 * t45610 + 0.5107751987195740728e-4_f64 * t45614 + 0.5107751987195740728e-4_f64 * t45617 + 0.11974241701863808564e0_f64 * t4041 * t10417 + 0.79828278012425390428e-1_f64 * t5928 * t9427 - 0.81300399444200075499e-3_f64 * t45630 - 0.162600798888400151e-2_f64 * t45633 - 0.81300399444200075499e-3_f64 * t45636 + 0.212822999466489197e-4_f64 * t45641 - 0.212822999466489197e-4_f64 * t45646;
    t48564
}
