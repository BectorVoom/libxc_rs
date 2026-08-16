//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2233/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2233(t7604: f64, t82573: f64, t3961: f64, t6746: f64, t11046: f64, t1409: f64, t14213: f64, t14571: f64, t14630: f64, t1629: f64, t23327: f64, t23511: f64, t23613: f64, t23633: f64, t23635: f64, t23657: f64, t23678: f64, t23685: f64, t25429: f64, t25540: f64, t25544: f64, t25717: f64, t25722: f64, t3120: f64, t4347: f64, t6687: f64, t6784: f64, t6797: f64, t6799: f64, t6800: f64, t7619: f64, t82661: f64, t83239: f64, t83240: f64, t83245: f64, t89019: f64) -> (f64, f64) {
    let t89104 = t82573 * t7604;
    let t89106 = t3961 * t6746;
    let t89143 = -0.48738787165873375897e-2_f64 * t89104 + 0.73108180748810063846e-2_f64 * t83239 * t83240 * t89106 + 0.10966227112321509577e-1_f64 * t83245 * t23511 * t1629 * t23678 * t14213 + t11046 * t7619 * t14630 + 0.54831135561607547884e-2_f64 * t6687 * t6784 * t23685 * t4347 - 0.18277045187202515961e-2_f64 * t82661 - 0.73108180748810063846e-2_f64 * t25429 * t89019 * t25722 + 0.82246703342411321825e-2_f64 * t6797 * t6799 * t14571 * t6800 - 0.54831135561607547884e-2_f64 * t23327 * t23613 * t25717 - 0.16449340668482264365e-1_f64 * t6797 * t23657 * t25540 - 0.16449340668482264365e-1_f64 * t6797 * t23657 * t25544 + 0.27415567780803773942e-2_f64 * t23633 * t23635 * t1409 * t3120 * t6800;
    (t89106, t89143)
}
