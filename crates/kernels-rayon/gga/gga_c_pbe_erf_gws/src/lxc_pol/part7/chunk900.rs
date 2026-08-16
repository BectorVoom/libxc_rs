//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 900/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk900(t11: f64, t17018: f64, t625: f64, t1416: f64, t1692: f64, t1243: f64, t1699: f64, t395: f64, t5074: f64, t5077: f64, t5071: f64, t5068: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t17020 = t11 * t625 * t17018;
    let t17022 = t1692 * t1416;
    let t17024 = t11 * t625 * t17022;
    let t17026 = t1243 * t1699;
    let t17028 = t395 * t5074;
    let t17030 = t395 * t5077;
    let t17032 = t395 * t5071;
    let t17034 = t395 * t5068;
    (t17020, t17022, t17024, t17026, t17028, t17030, t17032, t17034)
}
