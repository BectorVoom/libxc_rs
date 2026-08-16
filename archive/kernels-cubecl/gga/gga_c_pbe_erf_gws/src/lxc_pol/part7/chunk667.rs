//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 667/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk667<F: Float>(t1888: F, t5312: F, t1648: F, t1652: F, t1683: F, t633: F, t267: F, t5202: F, t5205: F, t5209: F, t5216: F, t5223: F, t5227: F, t5277: F, t5279: F, t5282: F, t5286: F, t5290: F, t5298: F, t5303: F, t5306: F, t5311: F) -> (F, F, F, F) {
    let t5314 = F::cast_from(8.0_f64) / F::cast_from(5.0_f64) * t5312 * t1888;
    let t5315 = t1648 * t1652;
    let t5316 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t5315;
    let t5317 = t633 * t1683;
    let t5318 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t5317;
    let t5319 = -t5202 * t267 / F::cast_from(15.0_f64) + F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t5205 + t5209 - t5216 - t5223 + t5227 - t5277 - t5279 - t5282 + t5286 + t5290 + t5298 + t5303 + t5306 - t5311 - t5314 + t5316 - t5318;
    (t5314, t5316, t5318, t5319)
}
