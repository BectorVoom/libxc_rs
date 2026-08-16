//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2246/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2246(t1920: f64, t25766: f64, t968: f64, t23384: f64, t25739: f64, t11010: f64, t12652: f64, t14552: f64, t14555: f64, t1603: f64, t1956: f64, t23327: f64, t23329: f64, t23571: f64, t25423: f64, t25429: f64, t25430: f64, t25743: f64, t25755: f64, t25767: f64, t3020: f64, t3169: f64, t3207: f64, t388: f64, t50632: f64, t6680: f64, t6687: f64, t6776: f64, t6816: f64, t7593: f64, t7625: f64, t986: f64) -> f64 {
    let t89561 = 0.54831135561607547884e-2_f64 * t1920 * t968 * t25766;
    let t89583 = 0.10966227112321509577e-1_f64 * t23384 * t25739;
    let t89590 = t89561 + 4.0_f64 * t14555 * t6776 + t3020 * t7593 * t388 - t25755 * t3207 - t11010 * t7625 + t1603 * t23571 * t388 - 0.43864908449286038306e-1_f64 * t6680 * t25767 + 4.0_f64 * t3169 * t25743 - 0.10966227112321509577e-1_f64 * t23327 * t23329 * t25423 * t12652 + 0.73108180748810063846e-2_f64 * t25429 * t23329 * t25430 * t12652 + t89583 - 0.16449340668482264365e-1_f64 * t6687 * t986 * t25766 - 2.0_f64 * t14552 * t6816 - t50632 * t1956;
    t89590
}
