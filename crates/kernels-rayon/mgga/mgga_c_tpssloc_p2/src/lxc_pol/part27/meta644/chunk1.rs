//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2199/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2199(t1409: f64, t14165: f64, t23327: f64, t23329: f64, t23402: f64, t25430: f64, t25442: f64, t25443: f64, t25750: f64, t25815: f64, t3175: f64, t6691: f64, t7557: f64, t82382: f64, t82402: f64, t82417: f64, t82502: f64, t88058: f64, t88069: f64, t88075: f64, t88076: f64, t88083: f64, t88089: f64, t88096: f64) -> f64 {
    let t88097 = 0.54831135561607547884e-2_f64 * t23327 * t82502 * t25750 - 0.54831135561607547884e-2_f64 * t23327 * t88058 * t6691 + 0.54831135561607547884e-2_f64 * t23327 * t25442 * t23402 - 0.54831135561607547884e-2_f64 * t23327 * t82417 * t25815 - t88069 + 0.16449340668482264365e-1_f64 * t23327 * t23329 * t25430 * t14165 - t88075 + 0.54831135561607547884e-2_f64 * t23327 * t23329 * t88076 * t1409 * t3175 - t88083 - 0.54831135561607547884e-2_f64 * t23327 * t82417 * t25750 + 0.14621636149762012769e-1_f64 * t82402 * t25443 - 0.54831135561607547884e-2_f64 * t23327 * t88089 * t6691 - 0.80418998823691070228e-1_f64 * t82382 * t7557 + t88096;
    t88097
}
