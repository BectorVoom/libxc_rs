//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1927/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1927(t1353: f64, t1398: f64, t13790: f64, t13789: f64, t3957: f64, t5690: f64, t1873: f64, t9741: f64, t5651: f64, t808: f64, t9736: f64, t241: f64, t820: f64, t9991: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13791 = t1353 * t1398;
    let t13792 = t13790 * t13791;
    let t13793 = t13789 * t13792;
    let t13797 = 7.0_f64 / 72.0_f64 * t3957 * t5690;
    let t13798 = t9741 * t1873;
    let t13800 = t808 * t5651;
    let t13801 = t9736 * t13800;
    let t13804 = t820 * t9991 * t241;
    (t13791, t13792, t13793, t13797, t13798, t13800, t13801, t13804)
}
