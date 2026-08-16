//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2230/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2230(t25608: f64, t6743: f64, t1948: f64, t6733: f64, t23631: f64, t61066: f64, t974: f64, t12652: f64, t14586: f64, t14595: f64, t23323: f64, t23327: f64, t23609: f64, t23657: f64, t23673: f64, t25502: f64, t25510: f64, t25511: f64, t25512: f64, t25523: f64, t6797: f64, t6799: f64, t6800: f64, t6801: f64, t7603: f64, t7615: f64, t82539: f64, t82555: f64, t82643: f64, t82657: f64) -> (f64, f64) {
    let t89002 = t25608 * t6743;
    let t89019 = t6733 * t1948;
    let t89033 = t23631 * t974 * t61066;
    let t89042 = -0.16449340668482264365e-1_f64 * t6797 * t89002 * t6801 - 0.10966227112321509577e-1_f64 * t23327 * t25510 * t25511 * t12652 + 0.54831135561607547884e-2_f64 * t82539 + 0.82246703342411321825e-2_f64 * t6797 * t6799 * t14595 * t6800 + 0.16449340668482264365e-1_f64 * t6797 * t6799 * t14586 * t6800 + 0.10966227112321509577e-1_f64 * t23327 * t89019 * t25512 - 0.16449340668482264365e-1_f64 * t6797 * t23657 * t25502 - 0.27415567780803773942e-2_f64 * t23327 * t82643 * t7603 - 0.18277045187202515961e-2_f64 * t82555 + 0.80418998823691070228e-1_f64 * t23323 * t7615 - 0.54831135561607547884e-2_f64 * t89033 * t82657 - 0.16449340668482264365e-1_f64 * t6797 * t25523 * t23609 - 0.82246703342411321825e-2_f64 * t6797 * t25523 * t23673;
    (t89019, t89042)
}
