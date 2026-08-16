//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 509/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk509<F: Float>(t9285: F, t9287: F, t1: F, t9078: F, t544: F, t2365: F, t6520: F, t7025: F, t9060: F, t9065: F, t1415: F, t2371: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t9289 = F::cast_from(0.29792074959875355558e-1_f64) * t9285 * t9287;
    let t9290 = t9078 * t1;
    let t9291 = t544 * t9290;
    let t9294 = t2365 * t6520;
    let t9296 = F::cast_from(0.29792074959875355558e-1_f64) * t7025 * t9294;
    let t9297 = t9060 * t1;
    let t9298 = t544 * t9297;
    let t9301 = t9065 * t1;
    let t9302 = t544 * t9301;
    let t9305 = t1415 * t2371;
    (t9289, t9290, t9291, t9294, t9296, t9297, t9298, t9301, t9302, t9305)
}
