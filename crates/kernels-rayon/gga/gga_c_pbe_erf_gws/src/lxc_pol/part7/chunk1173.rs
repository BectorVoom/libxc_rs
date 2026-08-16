//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1173/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1173(t20930: f64, t366: f64, t899: f64, t2157: f64, t20726: f64, t2264: f64, t2331: f64, t2268: f64, t2276: f64, t2299: f64, t6201: f64, t6581: f64) -> (f64, f64, f64, f64) {
    let t20932 = t899 * t20930 * t366;
    let t20933 = t2157 * t2157;
    let t20934 = t20726 * t20933;
    let t20940 = t899 * t2264 * t2331;
    let t20941 = t20940 * t2268;
    let t20944 = t2276 * t6201 * t2299;
    let t20945 = t20944 * t6581;
    (t20932, t20934, t20941, t20945)
}
