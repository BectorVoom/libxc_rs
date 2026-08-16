//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1429/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1429(t33261: f64, t36520: f64, t36521: f64, t36522: f64, t36523: f64, t36524: f64, t36526: f64, t36527: f64, t36528: f64, t36529: f64, t36530: f64, t33353: f64, t33375: f64, t33377: f64, t33380: f64, t36559: f64, t36560: f64, t36561: f64, t36562: f64, t36563: f64, t36564: f64, t36568: f64) -> (f64, f64) {
    let t38716 = -t36520 + t36521 + t36522 - t36523 - t36524 + 0.97817934710145362364e-6_f64 * t33261 + t36526 + t36527 + t36528 + t36529 + t36530;
    let t38726 = 0.90579542097823505428e-7_f64 * t33353 + t36559 + t36560 - t36561 + t36562 + t36563 + t36564 - 0.67632724766374884054e-5_f64 * t33375 - 0.54347725258694103258e-6_f64 * t33377 - 0.18115908419564701086e-6_f64 * t33380 - t36568;
    (t38716, t38726)
}
