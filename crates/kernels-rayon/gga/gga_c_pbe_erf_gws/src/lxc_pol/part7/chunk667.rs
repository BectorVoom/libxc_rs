//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 667/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk667(t1888: f64, t5312: f64, t1648: f64, t1652: f64, t1683: f64, t633: f64, t267: f64, t5202: f64, t5205: f64, t5209: f64, t5216: f64, t5223: f64, t5227: f64, t5277: f64, t5279: f64, t5282: f64, t5286: f64, t5290: f64, t5298: f64, t5303: f64, t5306: f64, t5311: f64) -> (f64, f64, f64, f64) {
    let t5314 = 8.0_f64 / 5.0_f64 * t5312 * t1888;
    let t5315 = t1648 * t1652;
    let t5316 = 16.0_f64 / 45.0_f64 * t5315;
    let t5317 = t633 * t1683;
    let t5318 = 8.0_f64 / 15.0_f64 * t5317;
    let t5319 = -t5202 * t267 / 15.0_f64 + 2.0_f64 / 45.0_f64 * t5205 + t5209 - t5216 - t5223 + t5227 - t5277 - t5279 - t5282 + t5286 + t5290 + t5298 + t5303 + t5306 - t5311 - t5314 + t5316 - t5318;
    (t5314, t5316, t5318, t5319)
}
