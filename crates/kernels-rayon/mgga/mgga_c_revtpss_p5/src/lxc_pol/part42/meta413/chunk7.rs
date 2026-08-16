//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1464/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1464(t22465: f64, t22473: f64, t22482: f64, t22504: f64, t1312: f64, t13426: f64, t1518: f64, t18220: f64, t18227: f64, t18245: f64, t21814: f64, t21881: f64, t2322: f64, t4248: f64, t4292: f64, t5523: f64, t5920: f64, t670: f64, t7889: f64) -> (f64, f64) {
    let t22506 = t22465 + t22473 + t22482 + t22504;
    let t22525 = 2.0_f64 * t1312 * t21881 + 4.0_f64 * t13426 * t1518 + 4.0_f64 * t1518 * t18227 + 2.0_f64 * t18245 * t670 + 2.0_f64 * t2322 * t5920 + 4.0_f64 * t4248 * t4292 + 4.0_f64 * t4292 * t7889 + 2.0_f64 * t5523 * t5920 + 2.0_f64 * t18220 + t21814;
    (t22506, t22525)
}
