//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 850/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk850(t3747: f64, t3861: f64, t905: f64, t274: f64, t3824: f64, t2157: f64, t1123: f64, t2255: f64, t1105: f64, t11478: f64, t2170: f64, t3138: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13393 = t3861 * t3747;
    let t13394 = t905 * t13393;
    let t13397 = t274 * t3824;
    let t13398 = t13397 * t2157;
    let t13400 = t2255 * t1123 * t13398;
    let t13403 = t2157 * t1105;
    let t13405 = t2170 * t11478 * t13403;
    let t13407 = t3138 * t13405 / 8.0_f64;
    (t13394, t13397, t13398, t13400, t13403, t13405, t13407)
}
