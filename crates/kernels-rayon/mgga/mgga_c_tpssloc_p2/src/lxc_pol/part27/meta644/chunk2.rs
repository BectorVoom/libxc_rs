//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2200/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2200(t23384: f64, t25785: f64, t25447: f64, t1625: f64, t6733: f64, t23328: f64, t6705: f64, t13742: f64, t1956: f64, t23327: f64, t23331: f64, t23346: f64, t23372: f64, t23728: f64, t25424: f64, t25429: f64, t25431: f64, t25757: f64, t25758: f64, t25810: f64, t4337: f64, t4342: f64, t4665: f64, t50622: f64, t6687: f64, t6691: f64, t82380: f64, t82502: f64) -> f64 {
    let t88100 = 0.54831135561607547884e-2_f64 * t23384 * t25785;
    let t88102 = 0.54831135561607547884e-2_f64 * t23384 * t25447;
    let t88105 = t6733 * t1625;
    let t88112 = t23328 * t6705;
    let t88137 = t88100 + t88102 - 0.43864908449286038306e-1_f64 * t23346 * t25785 - 0.54831135561607547884e-2_f64 * t23327 * t88105 * t6691 - 12.0_f64 * t25757 * t25758 * t13742 + 0.10966227112321509577e-1_f64 * t23327 * t88112 * t4342 * t23331 - 0.73108180748810063846e-2_f64 * t25429 * t88112 * t4337 * t23331 + 0.10966227112321509577e-1_f64 * t23327 * t82502 * t25424 - 0.73108180748810063846e-2_f64 * t25429 * t82502 * t25431 - 0.54831135561607547884e-2_f64 * t82380 + 4.0_f64 * t23372 * t4665 - 2.0_f64 * t50622 * t1956 - 0.43864908449286038306e-1_f64 * t23346 * t25447 + 0.27415567780803773942e-2_f64 * t6687 * t25810 * t23728;
    t88137
}
