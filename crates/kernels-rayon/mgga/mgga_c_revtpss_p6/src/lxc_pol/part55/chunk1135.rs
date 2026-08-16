//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1135/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1135(t120975: f64, t1401: f64, t1412: f64, t814: f64, t1372: f64, t32265: f64, t124: f64, t1380: f64, t1444: f64, t800: f64, t32705: f64, t239: f64, t8583: f64, t8589: f64, t9990: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t120976 = t120975 * t1401;
    let t120977 = 0.34708173928447610098e-2_f64 * t120976;
    let t120980 = t814 * t1412;
    let t120981 = t120980 * t1372;
    let t120982 = t32265 * t120981;
    let t120983 = 0.14874931683620404328e-3_f64 * t120982;
    let t120986 = t1380 * t800 * t124 * t1444;
    let t120987 = t32705 * t120986;
    let t120988 = 0.7437465841810202164e-4_f64 * t120987;
    let t120991 = t8583 * t8589 * t9990 * t239;
    (t120977, t120980, t120981, t120983, t120986, t120988, t120991)
}
