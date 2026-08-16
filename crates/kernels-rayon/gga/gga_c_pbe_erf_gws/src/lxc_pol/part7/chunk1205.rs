//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1205/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1205(t5: f64, t6439: f64, t343: f64, t2121: f64, t337: f64, t2134: f64, t2074: f64, t2122: f64, t2147: f64, t2120: f64, t20270: f64, t2276: f64) -> (f64, f64, f64, f64) {
    let t21419 = t5 * t6439;
    let t21420 = t21419 * t343;
    let t21422 = t2121 * t337 * t21420;
    let t21424 = t2134 * t21422 / 24.0_f64;
    let t21427 = t2147 * t337 * t2122 * t2074;
    let t21429 = t2120 * t21427 / 8.0_f64;
    let t21430 = t2276 * t20270;
    (t21419, t21424, t21429, t21430)
}
