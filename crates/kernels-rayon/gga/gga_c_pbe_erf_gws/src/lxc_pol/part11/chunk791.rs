//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 791/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk791(t3342: f64, t7087: f64, t1821: f64, t1820: f64, t1017: f64, t3425: f64, t1827: f64, t587: f64, t1044: f64, t3465: f64, t5522: f64, t639: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12809 = t7087 * t3342;
    let t12810 = t1821 * t12809;
    let t12812 = 16.0_f64 / 15.0_f64 * t1820 * t12810;
    let t12813 = t3425 * t1017;
    let t12814 = t1827 * t12813;
    let t12816 = 8.0_f64 / 15.0_f64 * t587 * t12814;
    let t12817 = t3465 * t1044;
    let t12818 = t5522 * t12817;
    let t12820 = 4.0_f64 / 9.0_f64 * t639 * t12818;
    (t12809, t12810, t12812, t12813, t12814, t12816, t12817, t12818, t12820)
}
