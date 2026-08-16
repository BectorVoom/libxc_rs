//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1457/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1457(t2439: f64, t6132: f64, t6135: f64, t6138: f64, t2873: f64, t6104: f64, t11108: f64, t6396: f64, t11452: f64, t6173: f64, t2986: f64, t6184: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t63533 = t2439 * t6132;
    let t63538 = t2439 * t6135;
    let t63545 = t2439 * t6138;
    let t63677 = t6104 * t2873;
    let t63907 = t6396 * t11108;
    let t63979 = t6173 * t11452;
    let t63997 = t6184 * t2986;
    (t63533, t63538, t63545, t63677, t63907, t63979, t63997)
}
