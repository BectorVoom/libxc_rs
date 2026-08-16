//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 215/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk215<F: Float>(t1232: F, t129: F, t453: F, t143: F, t463: F, t155: F, t462: F, t153: F, t122: F, t594: F, t169: F, t599: F) -> (F, F, F, F, F, F, F, F, F) {
    let pi = F::cast_from(M_PI);
    let t1233 = t1232 * t129;
    let t1234 = t453 * t453;
    let t1238 = t143 * t143;
    let t1240 = F::cast_from(1.0_f64) / t1238 / t143;
    let t1242 = t1240 * pi * t463;
    let t1246 = F::cast_from(1.0_f64) / t462 / t155;
    let t1247 = t153 * t1246;
    let t1338 = t122 * t594;
    let t1339 = t169 * t599;
    (t1233, t1234, t1238, t1240, t1242, t1246, t1247, t1338, t1339)
}
