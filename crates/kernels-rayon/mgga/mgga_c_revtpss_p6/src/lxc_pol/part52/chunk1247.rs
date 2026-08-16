//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1247/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1247(t32385: f64, t7732: f64, t2322: f64, t34196: f64, t4254: f64, t1936: f64, t28586: f64, t651: f64, t125478: f64, t125507: f64, t128317: f64, t128319: f64, t128321: f64, t128324: f64, t128326: f64, t128349: f64, t128354: f64, t128356: f64, t128357: f64, t128358: f64, t128360: f64, t128361: f64, t128362: f64, t128363: f64, t128478: f64, t28025: f64, t28287: f64, t28683: f64, t28734: f64, t28737: f64, t569: f64, t6985: f64, t7983: f64) -> f64 {
    let t128483 = 2.0_f64 * t7732 * t32385;
    let t128485 = 2.0_f64 * t2322 * t34196;
    let t128487 = 2.0_f64 * t4254 * t34196;
    let t128490 = 2.0_f64 * t651 * t28586 * t1936;
    let t128491 = -2.0_f64 * t6985 * t28737 + 2.0_f64 * t125478 * t28287 - 2.0_f64 * t6985 * t28734 - t128317 - t128319 - t128321 - t128324 + t128326 + (2.0_f64 * t28025 * t7983 + 2.0_f64 * t28683 * t6985 + 2.0_f64 * t128349 + 2.0_f64 * t128354 + 2.0_f64 * t128356 + 2.0_f64 * t128357 + 2.0_f64 * t128358 + 2.0_f64 * t128360 + 2.0_f64 * t128361 + 2.0_f64 * t128362 + 2.0_f64 * t128363 + t128478) * t569 - t125507 - t128483 - t128485 - t128487 - t128490;
    t128491
}
