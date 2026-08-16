//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 672/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk672(t203: f64, t5371: f64, t184: f64, t221: f64, t1750: f64, t663: f64, t172: f64, t1773: f64, t564: f64, t5324: f64, t5326: f64, t5328: f64, t5330: f64, t5332: f64, t5337: f64, t5339: f64, t5341: f64, t5345: f64, t5348: f64, t5350: f64, t5354: f64, t5356: f64, t5359: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5372 = t203 * t5371;
    let t5373 = t5372 * t184;
    let t5375 = 2.0_f64 / 15.0_f64 * t5373 * t221;
    let t5377 = 2.0_f64 / 5.0_f64 * t1750 * t663;
    let t5378 = t172 * t1773;
    let t5379 = t5378 * t184;
    let t5381 = 4.0_f64 / 5.0_f64 * t5379 * t564;
    let t5382 = -t5324 - t5326 - t5328 + t5330 + t5332 + t5337 + t5339 + t5341 + t5345 + t5348 + t5350 + t5354 - t5356 + t5359 + t5375 - t5377 + t5381;
    (t5372, t5373, t5375, t5377, t5378, t5379, t5381, t5382)
}
