//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 618/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk618(t191: f64, t784: f64, t190: f64, t212: f64, t205: f64, t626: f64, t1641: f64, t261: f64, t174: f64, t838: f64, t1639: f64, t56: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5044 = t784 * t191;
    let t5047 = 0.29629629629629629629e-1_f64 * t190 * t5044 * t212;
    let t5060 = 1.0_f64 / t205 / t626;
    let t5061 = t191 * t5060;
    let t5063 = 1.0_f64 / t1641 / t261;
    let t5081 = t174 * t838 * t205;
    let t5082 = 0.11197407407407407407e0_f64 * t5081;
    let t5089 = t56 * t1639;
    (t5044, t5047, t5061, t5063, t5081, t5082, t5089)
}
