//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1491/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1491(t3940: f64, t9962: f64, t1371: f64, t3889: f64, t800: f64, t221: f64, t3924: f64, t4019: f64, t4018: f64, t3930: f64, t4059: f64, t1386: f64, t2482: f64, t596: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9963 = t9962 * t3940;
    let t9966 = t800 * t1371 * t3889;
    let t9970 = t4019 * t221 * t3924;
    let t9971 = t4018 * t9970;
    let t9973 = t3930 * t4059;
    let t9976 = t2482 * t1386 * t596;
    (t9963, t9966, t9970, t9971, t9973, t9976)
}
