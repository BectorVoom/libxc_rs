//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 778/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk778(t12609: f64, t10697: f64, t3247: f64, t9647: f64, t13019: f64, t2580: f64, t2508: f64, t13023: f64, t1024: f64, t3266: f64, t2936: f64, t3255: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13198 = 0.1281754371690370714e-2_f64 * t12609;
    let t13200 = t10697 * t3247;
    let t13201 = t9647 * t13200;
    let t13202 = 0.1922631557535556071e-2_f64 * t13201;
    let t13203 = t2580 * t13019;
    let t13204 = t2508 * t13203;
    let t13206 = t2580 * t13023;
    let t13208 = 0.15381052460284448567e-1_f64 * t2508 * t13206;
    let t13209 = t3266 * t1024;
    let t13211 = 0.76905262301422242837e-2_f64 * t2508 * t13209;
    let t13212 = t2936 * t3255;
    (t13198, t13200, t13202, t13203, t13204, t13206, t13208, t13209, t13211, t13212)
}
