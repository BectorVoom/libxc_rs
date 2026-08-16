//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2839/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2839(t1041: f64, t1046: f64, t42994: f64, t1086: f64, t11213: f64, t3090: f64, t3057: f64, t3316: f64, t4891: f64, t3298: f64, t3059: f64, t3154: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t42996 = t1041 * t42994 * t1046;
    let t43038 = t11213 * t1086 * t3090;
    let t43043 = t3057 * t3316;
    let t43044 = t43043 * t4891;
    let t43049 = t3057 * t3298;
    let t43050 = t43049 * t4891;
    let t43051 = t3154 * t3059;
    (t42996, t43038, t43043, t43044, t43049, t43050, t43051)
}
