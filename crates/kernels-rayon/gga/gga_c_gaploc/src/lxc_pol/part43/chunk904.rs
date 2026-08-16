//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 904/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk904(t13055: f64, t28073: f64, t32840: f64, t3295: f64, t9805: f64, t11053: f64, t9829: f64, t20671: f64, t28856: f64, t32847: f64, t40956: f64, t13058: f64, t28737: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43370 = t28073 * t13055;
    let t43371 = 0.11502877786176224903e1_f64 * t43370;
    let t43373 = t9805 * t32840 * t3295;
    let t43374 = 0.11502877786176224903e1_f64 * t43373;
    let t43377 = t9805 * t11053 * t9829;
    let t43378 = 0.11502877786176224903e1_f64 * t43377;
    let t43383 = t28856 * t20671 * t32847;
    let t43384 = 0.25561950635947166451e0_f64 * t43383;
    let t43385 = 0.23005755572352449806e1_f64 * t40956;
    let t43386 = t28737 * t13058;
    (t43371, t43374, t43378, t43384, t43385, t43386)
}
