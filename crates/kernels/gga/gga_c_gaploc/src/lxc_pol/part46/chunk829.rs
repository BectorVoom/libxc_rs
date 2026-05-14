//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 829/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk829<F: Float>(t43373: F, t11053: F, t9805: F, t9829: F, t20671: F, t28856: F, t32847: F, t40956: F, t13058: F, t28737: F, t33289: F, t9800: F, t9806: F, t43007: F, t5241: F, t5640: F, t590: F) -> (F, F, F, F, F, F, F) {
    let t43374 = 0.11502877786176224903e1 * t43373;
    let t43377 = t9805 * t11053 * t9829;
    let t43378 = 0.11502877786176224903e1 * t43377;
    let t43383 = t28856 * t20671 * t32847;
    let t43384 = 0.25561950635947166451e0 * t43383;
    let t43385 = 0.23005755572352449806e1 * t40956;
    let t43386 = t28737 * t13058;
    let t43387 = 0.76685851907841499353e0 * t43386;
    let t43389 = t9800 * t33289 * t9806;
    let t43390 = 0.72851559312449424385e1 * t43389;
    let t43393 = t5640 * t5241 * t43007 * t590;
    (t43374, t43378, t43384, t43385, t43387, t43390, t43393)
}
