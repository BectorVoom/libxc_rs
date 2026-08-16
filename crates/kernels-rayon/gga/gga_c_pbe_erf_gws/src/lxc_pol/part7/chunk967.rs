//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 967/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk967(t1620: f64, t4878: f64, t4934: f64, t16984: f64, t16960: f64, t16962: f64, t16993: f64, t17003: f64, t17007: f64, t17020: f64, t17026: f64, t17028: f64, t17034: f64, t17040: f64) -> (f64, f64) {
    let t17883 = t1620 * t4934 * t4878;
    let t17900 = 0.37324691358024691357e0_f64 * t16984;
    let t17911 = t17900 - 0.71983333333333333335e-1_f64 * t17007 + 0.8638e0_f64 * t17020 + 0.19195555555555555555e0_f64 * t17026 + 0.28793333333333333333e0_f64 * t17028 - 0.19195555555555555556e0_f64 * t17034 + 0.4798888888888888889e0_f64 * t17040 + 0.28793333333333333333e0_f64 * t16993 + 0.86380000000000000002e0_f64 * t17003 + 0.14929876543209876543e0_f64 * t16960 - 0.95977777777777777776e-1_f64 * t16962;
    (t17883, t17911)
}
