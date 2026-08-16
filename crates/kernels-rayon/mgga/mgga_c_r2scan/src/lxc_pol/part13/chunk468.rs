//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 468/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk468(t481: f64, t537: f64, t495: f64, t2124: f64, t1625: f64, t1629: f64, t1635: f64, t1638: f64, t2080: f64, t2083: f64, t2088: f64, t2095: f64, t2108: f64, t2119: f64, t2122: f64, t279: f64, t535: f64, t574: f64) -> (f64, f64) {
    let t2125 = t537 * t481;
    let t2126 = t2125 * t495;
    let t2127 = t2124 * t2126;
    let t2130 = -0.27439371595564631661e-1_f64 * t535 * t1625 - 0.27439371595564631661e-1_f64 * t535 * t1629 + 0.23115257973478049502e0_f64 * t1635 - 0.43341108700271342816e-1_f64 * t574 * t1638 + 0.43341108700271342816e-1_f64 * t2080 * t279 - 0.23115257973478049502e0_f64 * t2083 + t2088 + t2095 + t2108 + t2119 + 0.10975748638225852664e0_f64 * t2122 * t2127;
    (t2127, t2130)
}
