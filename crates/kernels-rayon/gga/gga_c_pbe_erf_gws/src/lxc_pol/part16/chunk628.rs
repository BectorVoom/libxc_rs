//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 628/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk628(t532: f64, t991: f64, t159: f64, t285: f64, t143: f64, t2873: f64, t125: f64, t1493: f64, t1499: f64, t1501: f64, t169: f64, t2033: f64, t2035: f64, t2037: f64, t281: f64, t2848: f64, t2932: f64, t2937: f64, t2939: f64, t296: f64, t2984: f64, t2986: f64, t299: f64, t2990: f64, t301: f64, t3011: f64, t475: f64, t988: f64) -> (f64, f64) {
    let t3013 = t532 * t991;
    let t3015 = t3013 * t159 * t285;
    let t3021 = t143 * t2873;
    let t3024 = -0.11974234010254609094e-1_f64 * t1493 - t1499 + t988 * t1501 - 0.11974234010254609094e-1_f64 * t281 * t2932 - 0.11974234010254609094e-1_f64 * t2937 + 0.19957056683757681823e-1_f64 * t2939 + t2984 * t125 + 3.0_f64 * t2986 * t2037 - t988 * t2033 + 3.0_f64 * t2035 * t2990 + t3011 * t296 - 0.29056741517886919367e-3_f64 * t3015 + 0.20267214298646782767e-1_f64 * t169 * t299 * t2848 * t301 + 3.0_f64 * t475 * t3021;
    (t3013, t3024)
}
