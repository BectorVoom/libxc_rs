//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1197/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1197(t14102: f64, t51382: f64, t2153: f64, t2331: f64, t899: f64, t918: f64, t864: f64, t935: f64, t14058: f64, t2302: f64, t1477: f64, t360: f64, t56: f64, t863: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t51383 = t51382 * t14102;
    let t51387 = t899 * t2153 * t2331;
    let t51388 = t51387 * t918;
    let t51395 = t899 * t864 * t2331;
    let t51396 = t51395 * t935;
    let t51401 = t14058 * t2302;
    let t51407 = t863 * t360 * t1477 * t56;
    (t51383, t51387, t51388, t51395, t51396, t51401, t51407)
}
