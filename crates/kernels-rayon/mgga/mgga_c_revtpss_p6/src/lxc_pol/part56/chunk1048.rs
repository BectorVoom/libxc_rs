//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1048/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1048(t120981: f64, t32265: f64, t124: f64, t1380: f64, t1444: f64, t800: f64, t32705: f64, t239: f64, t8583: f64, t8589: f64, t9990: f64, t1405: f64, t32272: f64) -> (f64, f64, f64, f64, f64) {
    let t120982 = t32265 * t120981;
    let t120986 = t1380 * t800 * t124 * t1444;
    let t120987 = t32705 * t120986;
    let t120991 = t8583 * t8589 * t9990 * t239;
    let t120994 = t32272 * t1405;
    (t120982, t120986, t120987, t120991, t120994)
}
