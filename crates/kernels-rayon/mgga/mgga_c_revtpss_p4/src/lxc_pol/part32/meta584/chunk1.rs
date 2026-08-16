//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1913/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1913(t1385: f64, t8085: f64, t1903: f64, t26304: f64, t28925: f64, t531: f64, t2411: f64, t28455: f64, t198: f64, t206: f64, t8019: f64, t28309: f64, t686: f64, t72: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t102656 = t1385 * t8085;
    let t102661 = t26304 * t1903;
    let t102769 = t531 * t28925;
    let t102854 = t28455 * t2411;
    let t102888 = t198 * t206 * t8019;
    let t102928 = t28309 * t72 * t686;
    (t102656, t102661, t102769, t102854, t102888, t102928)
}
