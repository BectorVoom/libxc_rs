//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 871/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk871<F: Float>(t43383: F, t40956: F, t13058: F, t28737: F, t33289: F, t9800: F, t9806: F, t43007: F, t5241: F, t5640: F, t590: F, t43107: F, t11068: F, t2679: F, t9796: F, t33308: F, t9805: F) -> (F, F, F, F, F, F, F, F) {
    let t43384 = 0.25561950635947166451e0 * t43383;
    let t43385 = 0.23005755572352449806e1 * t40956;
    let t43386 = t28737 * t13058;
    let t43387 = 0.76685851907841499353e0 * t43386;
    let t43389 = t9800 * t33289 * t9806;
    let t43390 = 0.72851559312449424385e1 * t43389;
    let t43393 = t5640 * t5241 * t43007 * t590;
    let t43398 = 0.15337170381568299871e1 * t5640 * t5241 * t43107 * t590;
    let t43400 = t9796 * t11068 * t2679;
    let t43401 = 0.15337170381568299871e1 * t43400;
    let t43403 = t9805 * t33308 * t9806;
    (t43384, t43385, t43387, t43390, t43393, t43398, t43401, t43403)
}
