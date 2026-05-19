//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 619/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk619<F: Float>(t2299: F, t986: F, t1415: F, t1646: F, t2877: F, t9285: F, t3390: F, t4614: F, t574: F, t3354: F, t597: F, t2437: F) -> (F, F, F, F, F, F) {
    let t10318 = t2299 * t986;
    let t10319 = t1415 * t10318;
    let t10321 = F::cast_from(0.35750489951850426669e0_f64) * t10319 * t1646;
    let t10323 = F::cast_from(0.35750489951850426669e0_f64) * t9285 * t2877;
    let t10324 = t4614 * t3390;
    let t10326 = F::cast_from(0.61348681526273199483e1_f64) * t574 * t10324;
    let t10327 = t4614 * t3354;
    let t10329 = F::cast_from(0.15337170381568299871e2_f64) * t597 * t10327;
    let t10331 = F::cast_from(0.35750489951850426669e0_f64) * t2437 * t2877;
    (t10318, t10321, t10323, t10326, t10329, t10331)
}
