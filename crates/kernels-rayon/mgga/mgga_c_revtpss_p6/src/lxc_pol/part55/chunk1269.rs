//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1269/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1269(t2014: f64, t32734: f64, t5542: f64, t4292: f64, t651: f64, t8686: f64, t32385: f64, t4248: f64, t27123: f64, t8641: f64, t27126: f64, t32401: f64, t7732: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t128983 = t2014 * t32734 * t5542;
    let t128986 = t651 * t8686 * t4292;
    let t128988 = t4248 * t32385;
    let t128990 = t27123 * t8641;
    let t128992 = t27126 * t8641;
    let t128994 = t7732 * t32401;
    (t128983, t128986, t128988, t128990, t128992, t128994)
}
