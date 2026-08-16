//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1023/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1023(t12840: f64, t401: f64, t12846: f64, t12855: f64, t12858: f64, t12837: f64, t12843: f64, t12527: f64, t586: f64, t10419: f64, t2753: f64, t11032: f64, t2640: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t41888 = t401 * t12840;
    let t41890 = t401 * t12846;
    let t41939 = t401 * t12855;
    let t41941 = t401 * t12858;
    let t41974 = t401 * t12837;
    let t41976 = t401 * t12843;
    let t42011 = t12527 * t586;
    let t42014 = t10419 * t2753;
    let t42037 = t11032 * t2640;
    (t41888, t41890, t41939, t41941, t41974, t41976, t42011, t42014, t42037)
}
