//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1013/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1013<F: Float>(t43386: F, t33289: F, t9800: F, t9806: F, t43007: F, t5241: F, t5640: F, t590: F, t43107: F, t11068: F, t2679: F, t9796: F) -> (F, F, F, F, F) {
    let t43387 = F::cast_from(0.76685851907841499353e0_f64) * t43386;
    let t43389 = t9800 * t33289 * t9806;
    let t43390 = F::cast_from(0.72851559312449424385e1_f64) * t43389;
    let t43393 = t5640 * t5241 * t43007 * t590;
    let t43398 = F::cast_from(0.15337170381568299871e1_f64) * t5640 * t5241 * t43107 * t590;
    let t43400 = t9796 * t11068 * t2679;
    (t43387, t43390, t43393, t43398, t43400)
}
