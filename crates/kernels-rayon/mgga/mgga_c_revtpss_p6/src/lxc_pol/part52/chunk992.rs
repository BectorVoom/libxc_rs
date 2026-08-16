//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 992/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk992(t1843: f64, t7373: f64, t118: f64, t13426: f64, t1502: f64, t18227: f64, t1911: f64, t2052: f64, t2056: f64, t2089: f64, t25082: f64, t28196: f64, t28287: f64, t28586: f64, t28588: f64, t28653: f64, t28686: f64, t4246: f64, t4248: f64, t5517: f64, t569: f64, t651: f64, t671: f64, t7357: f64, t7367: f64, t7474: f64, t7484: f64) -> (f64, f64) {
    let t28696 = t1843 * t7373;
    let t28699 = -t118 * t28586 - 2.0_f64 * t13426 * t2056 - t1502 * t7474 - 2.0_f64 * t18227 * t2056 - t1843 * t7357 + t1911 * t7484 - t2052 * t5517 - t2089 * t4246 - 3.0_f64 * t25082 * t28588 + 2.0_f64 * t28196 * t28287 - 2.0_f64 * t28653 * t671 + t28686 * t569 - 2.0_f64 * t28696 * t651 - 2.0_f64 * t4248 * t7367;
    (t28696, t28699)
}
