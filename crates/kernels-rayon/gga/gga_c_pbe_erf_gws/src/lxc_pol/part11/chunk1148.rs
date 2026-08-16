//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1148/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1148(t41847: f64, t47377: f64, t5003: f64, t639: f64, t642: f64, t2677: f64, t47969: f64, t47975: f64, t10636: f64, t10646: f64, t108: f64, t12345: f64, t12355: f64, t1523: f64, t1528: f64, t25230: f64, t2538: f64, t2544: f64, t267: f64, t3346: f64, t3354: f64, t47372: f64, t47391: f64, t47400: f64, t47409: f64, t47733: f64, t48261: f64, t48265: f64, t48267: f64, t48270: f64, t48272: f64, t48274: f64, t726: f64, t728: f64, t92: f64, t93: f64) -> (f64, f64, f64, f64, f64) {
    let t48275 = 64.0_f64 / 45.0_f64 * t41847;
    let t48279 = 32.0_f64 / 15.0_f64 * t639 * t642 * t5003 * t47377;
    let t48282 = 16.0_f64 / 3.0_f64 * t639 * t2677 * t47969;
    let t48285 = 16.0_f64 / 27.0_f64 * t639 * t2677 * t47975;
    let t48286 = -(-40.0_f64 / 81.0_f64 * t1523 * t47391 + 80.0_f64 / 9.0_f64 * t10636 * t3346 + 20.0_f64 / 3.0_f64 * t92 * t47409 + 80.0_f64 / 9.0_f64 * t2538 * t12345 + 4.0_f64 / 3.0_f64 * t726 * t47400 - 40.0_f64 / 81.0_f64 * t1528 * t47377 + 80.0_f64 / 9.0_f64 * t10646 * t3354 + 20.0_f64 / 3.0_f64 * t93 * t47733 + 80.0_f64 / 9.0_f64 * t2544 * t12355 + 4.0_f64 / 3.0_f64 * t728 * t47372) * t108 * t267 / 15.0_f64 + t48261 + t48265 - t48267 + 0.39894533333333333332e0_f64 * t25230 + t48270 + t48272 + t48274 - t48275 - t48279 - t48282 + t48285;
    (t48275, t48279, t48282, t48285, t48286)
}
