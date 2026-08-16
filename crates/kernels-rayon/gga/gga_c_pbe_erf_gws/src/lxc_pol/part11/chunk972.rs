//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 972/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk972(t1243: f64, t3466: f64, t3474: f64, t3470: f64, t17791: f64, t3406: f64, t639: f64, t3443: f64, t5219: f64, t10968: f64, t586: f64, t1672: f64, t211: f64, t3554: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t30955 = t1243 * t3466;
    let t30957 = t1243 * t3474;
    let t30962 = t1243 * t3470;
    let t31102 = t639 * t17791 * t3406;
    let t31133 = t5219 * t3443;
    let t31168 = t10968 * t586;
    let t31200 = t211 * t1672 * t3554;
    (t30955, t30957, t30962, t31102, t31133, t31168, t31200)
}
