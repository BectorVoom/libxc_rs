//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1239/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1239(t28042: f64, t7359: f64, t108120: f64, t122570: f64, t125362: f64, t125365: f64, t127535: f64, t128291: f64, t128331: f64, t128332: f64, t128333: f64, t128335: f64, t128337: f64, t128338: f64, t128339: f64, t128340: f64, t1518: f64, t2055: f64, t25805: f64, t28030: f64, t32389: f64, t33602: f64, t4292: f64, t670: f64, t7373: f64, t7983: f64, t97622: f64) -> f64 {
    let t128341 = t7359 * t28042;
    let t128349 = t108120 * t2055 + t122570 * t1518 + t125362 * t2055 + t125365 * t2055 + t127535 * t1518 + t128291 * t670 + t2055 * t97622 + t25805 * t7983 + t28030 * t7373 + t32389 * t4292 + t33602 * t7373 + t128331 + t128332 + t128333 + t128335 + t128337 + t128338 + t128339 + t128340 + t128341;
    t128349
}
