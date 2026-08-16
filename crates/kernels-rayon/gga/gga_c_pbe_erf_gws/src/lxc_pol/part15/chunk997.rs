//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 997/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk997(t6659: f64, t858: f64, t8939: f64, t884: f64, t2079: f64, t2112: f64, t326: f64, t3107: f64, t860: f64, t2119: f64, t3039: f64, t2124: f64) -> (f64, f64, f64, f64) {
    let t8941 = t6659 * t858 * t8939;
    let t8943 = t884 * t8941 / 4.0_f64;
    let t8944 = t2079 * t2112;
    let t8945 = t326 * t8944;
    let t8946 = t8945 * t3107;
    let t8948 = t8946 * t860 / 96.0_f64;
    let t8949 = t3039 * t2119;
    let t8951 = t8949 * t2124 / 48.0_f64;
    (t8943, t8945, t8948, t8951)
}
