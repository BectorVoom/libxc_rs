//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 893/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk893(t197: f64, t4991: f64, t1661: f64, t1802: f64, t5480: f64, t649: f64, t16984: f64, t1697: f64, t191: f64, t205: f64, t190: f64, t212: f64, t367: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17819 = t4991 * t197;
    let t17852 = t1661 * t1802;
    let t17870 = t5480 * t649;
    let t17900 = 0.37324691358024691357e0_f64 * t16984;
    let t17957 = t191 / t205 / t1697;
    let t17983 = 0.10864197530864197531e0_f64 * t190 * t367 * t212;
    (t17819, t17852, t17870, t17900, t17957, t17983)
}
