//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1023/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1023(t15688: f64, t3299: f64, t1678: f64, t3057: f64, t379: f64, t1078: f64, t1651: f64, t3286: f64, t4746: f64, t1647: f64, t3298: f64, t1086: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16226 = t3299 * t15688;
    let t16284 = t3057 * t1678;
    let t16312 = t3057 * t379;
    let t16313 = t1078 * t1651;
    let t16502 = t4746 * t3286;
    let t16509 = t1647 * t3298;
    let t16543 = t1086 * t1678;
    (t16226, t16284, t16312, t16313, t16502, t16509, t16543)
}
