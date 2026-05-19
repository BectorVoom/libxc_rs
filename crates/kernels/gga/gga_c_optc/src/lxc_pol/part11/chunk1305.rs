//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1305/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1305<F: Float>(t10416: F, t16655: F, t16933: F, t3788: F, t13842: F, t41392: F, t845: F, t16929: F, t24021: F, t56677: F, t7504: F, t2416: F, t4815: F, t4818: F) -> (F, F, F, F, F, F) {
    let t57246 = F::new(24.0) * t10416 * t16655;
    let t57248 = F::cast_from(0.14035736153892489771e2_f64) * t3788 * t16933;
    let t57251 = F::cast_from(0.61523382126046769581e4_f64) * t845 * t13842 * t41392;
    let t57253 = F::cast_from(0.4155781415850207192e3_f64) * t3788 * t16929;
    let t57257 = F::cast_from(0.12304676425209353917e5_f64) * t845 * t24021 * t56677 * t7504;
    let t57260 = F::new(36.0) * t2416 * t4815 * t4818;
    (t57246, t57248, t57251, t57253, t57257, t57260)
}
