//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1171/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1171<F: Float>(t16943: F, t3788: F, t1375: F, t49995: F, t23801: F, t23804: F, t56677: F, t845: F, t10416: F, t16655: F, t16933: F, t13842: F, t41392: F, t16929: F, t24021: F, t7504: F) -> (F, F, F, F, F, F, F, F) {
    let t57238 = 0.41015588084031179722e4 * t3788 * t16943;
    let t57240 = 0.23392893589820816284e1 * t49995 * t1375;
    let t57244 = 0.91080982599109921211e5 * t845 * t23801 * t56677 * t23804;
    let t57246 = 24.0 * t10416 * t16655;
    let t57248 = 0.14035736153892489771e2 * t3788 * t16933;
    let t57251 = 0.61523382126046769581e4 * t845 * t13842 * t41392;
    let t57253 = 0.4155781415850207192e3 * t3788 * t16929;
    let t57257 = 0.12304676425209353917e5 * t845 * t24021 * t56677 * t7504;
    (t57238, t57240, t57244, t57246, t57248, t57251, t57253, t57257)
}
