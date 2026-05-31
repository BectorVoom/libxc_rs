//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1148/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1148<F: Float>(t41847: F, t47377: F, t5003: F, t639: F, t642: F, t2677: F, t47969: F, t47975: F, t10636: F, t10646: F, t108: F, t12345: F, t12355: F, t1523: F, t1528: F, t25230: F, t2538: F, t2544: F, t267: F, t3346: F, t3354: F, t47372: F, t47391: F, t47400: F, t47409: F, t47733: F, t48261: F, t48265: F, t48267: F, t48270: F, t48272: F, t48274: F, t726: F, t728: F, t92: F, t93: F) -> (F, F, F, F, F) {
    let t48275 = F::cast_from(64.0_f64) / F::cast_from(45.0_f64) * t41847;
    let t48279 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t639 * t642 * t5003 * t47377;
    let t48282 = F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t639 * t2677 * t47969;
    let t48285 = F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t639 * t2677 * t47975;
    let t48286 = -(-F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t1523 * t47391 + F::cast_from(80.0_f64) / F::cast_from(9.0_f64) * t10636 * t3346 + F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t92 * t47409 + F::cast_from(80.0_f64) / F::cast_from(9.0_f64) * t2538 * t12345 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t726 * t47400 - F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t1528 * t47377 + F::cast_from(80.0_f64) / F::cast_from(9.0_f64) * t10646 * t3354 + F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t93 * t47733 + F::cast_from(80.0_f64) / F::cast_from(9.0_f64) * t2544 * t12355 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t728 * t47372) * t108 * t267 / F::cast_from(15.0_f64) + t48261 + t48265 - t48267 + F::cast_from(0.39894533333333333332e0_f64) * t25230 + t48270 + t48272 + t48274 - t48275 - t48279 - t48282 + t48285;
    (t48275, t48279, t48282, t48285, t48286)
}
