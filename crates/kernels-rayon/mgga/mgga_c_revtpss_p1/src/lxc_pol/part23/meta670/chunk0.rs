//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2404/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2404(t1041: f64, t1046: f64, t42994: f64, t3057: f64, t3316: f64, t4891: f64, t3298: f64, t11670: f64, t11772: f64, t3114: f64, t11773: f64, t11926: f64) -> (f64, f64, f64, f64, f64) {
    let t42996 = t1041 * t42994 * t1046;
    let t43043 = t3057 * t3316;
    let t43044 = t43043 * t4891;
    let t43049 = t3057 * t3298;
    let t43050 = t43049 * t4891;
    let t43065 = t11670 * t11772;
    let t43066 = t3114 * t43065;
    let t43069 = t11926 * t11773;
    (t42996, t43044, t43050, t43066, t43069)
}
