//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 864/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk864(t3074: f64, t8847: f64, t2118: f64, t3106: f64, t3178: f64, t810: f64, t337: f64, t6560: f64, t814: f64, t2147: f64, t360: f64, t2306: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8848 = t3074 * t8847;
    let t8860 = t2118 * t3106;
    let t8867 = t3178 * t810;
    let t8868 = t337 * t8867;
    let t8869 = t6560 * t8868;
    let t8873 = t337 * t3178 * t814;
    let t8874 = t2147 * t8873;
    let t8879 = t3106 * t360;
    let t8880 = t2306 * t8879;
    (t8848, t8860, t8867, t8869, t8874, t8880)
}
