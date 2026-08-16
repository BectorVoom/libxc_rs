//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 449/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk449(t2435: f64, t2439: f64, t2502: f64, t2504: f64, t2509: f64, t2511: f64, t730: f64, t722: f64, t164: f64, t172: f64, t2538: f64, t123: f64, t147: f64, t2434: f64) -> (f64, f64, f64, f64) {
    let t2548 = -0.78438333333333333333e0_f64 * t2502 + 0.15687666666666666667e1_f64 * t2504 + 0.68863333333333333333e0_f64 * t2435 + 0.14025833333333333333e0_f64 * t2509 + 0.28051666666666666667e0_f64 * t2511 + 0.17365833333333333333e0_f64 * t2439;
    let t2549 = t2548 * t730;
    let t2552 = t722 * t722;
    let t2553 = 1.0_f64 / t2552;
    let t2554 = t164 * t2553;
    let t2555 = t172 * t172;
    let t2556 = 1.0_f64 / t2555;
    let t2557 = t2538 * t2556;
    let t2562 = 0.14764627977777777777e-2_f64 * t123 * t2434 * t147;
    (t2549, t2554, t2557, t2562)
}
