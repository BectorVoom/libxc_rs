//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1004/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1004(t12439: f64, t1620: f64, t5493: f64, t12443: f64, t12782: f64, t5137: f64, t639: f64, t12486: f64, t583: f64, t12589: f64, t185: f64, t582: f64) -> (f64, f64, f64, f64, f64) {
    let t39883 = t1620 * t5493 * t12439;
    let t39886 = t1620 * t5493 * t12443;
    let t39931 = t639 * t5137 * t12782;
    let t39951 = t12486 * t583;
    let t40039 = t185 * t582 * t12589;
    (t39883, t39886, t39931, t39951, t40039)
}
