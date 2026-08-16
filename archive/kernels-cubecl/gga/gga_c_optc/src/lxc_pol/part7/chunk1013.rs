//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1013/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1013<F: Float>(t108: F, t117: F, t22154: F, t56: F, t127: F, t616: F, t6867: F, t2034: F, t2030: F, t6933: F, t6: F, t9771: F) -> (F, F, F, F, F) {
    let t22158 = F::cast_from(455.0_f64) / F::cast_from(243.0_f64) * t108 * t22154 * t56 * t117;
    let t22160 = t6867 * t127 * t616;
    let t22161 = t2034 * t22160;
    let t22164 = t2030 * t6933;
    let t22166 = t9771 * t6;
    (t22158, t22160, t22161, t22164, t22166)
}
