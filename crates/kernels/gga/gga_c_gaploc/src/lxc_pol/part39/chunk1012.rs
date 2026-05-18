//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1012/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1012<F: Float>(t43370: F, t32840: F, t3295: F, t9805: F, t11053: F, t9829: F, t20671: F, t28856: F, t32847: F, t40956: F, t13058: F, t28737: F) -> (F, F, F, F, F, F) {
    let t43371 = F::new(0.11502877786176224903e1) * t43370;
    let t43373 = t9805 * t32840 * t3295;
    let t43374 = F::new(0.11502877786176224903e1) * t43373;
    let t43377 = t9805 * t11053 * t9829;
    let t43378 = F::new(0.11502877786176224903e1) * t43377;
    let t43383 = t28856 * t20671 * t32847;
    let t43384 = F::new(0.25561950635947166451e0) * t43383;
    let t43385 = F::new(0.23005755572352449806e1) * t40956;
    let t43386 = t28737 * t13058;
    (t43371, t43374, t43378, t43384, t43385, t43386)
}
