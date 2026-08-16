//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 939/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk939(t285: f64, t3013: f64, t545: f64, t39: f64, t991: f64, t159: f64, t2522: f64, t532: f64, t143: f64, t1501: f64, t169: f64, t279: f64, t281: f64, t2857: f64, t2922: f64, t299: f64, t301: f64, t475: f64, t481: f64, t526: f64, t8038: f64, t8061: f64, t8075: f64, t8102: f64, t8108: f64, t8112: f64, t8258: f64, t8261: f64, t8267: f64, t8270: f64, t8275: f64) -> f64 {
    let t8277 = t3013 * t545 * t285;
    let t8279 = t39 * t991;
    let t8281 = t8279 * t159 * t285;
    let t8287 = t532 * t2522;
    let t8290 = 0.58113483035773838734e-3_f64 * t8287 * t159 * t285;
    let t8291 = (t8061 + t8075) * t279 + 3.0_f64 * t475 * t143 * t8102 + 2.0_f64 * t2922 * t1501 + 12.0_f64 * t2857 * t8108 * t481 + 6.0_f64 * t2857 * t8112 + t8258 * t526 - 0.11974234010254609094e-1_f64 * t281 * t8261 - t8267 - 0.11974234010254609094e-1_f64 * t8270 - t8275 - 0.58113483035773838734e-3_f64 * t8277 + 0.13559812708347229038e-2_f64 * t8281 + 0.20267214298646782767e-1_f64 * t169 * t299 * t8038 * t301 - t8290;
    t8291
}
