//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 816/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk816<F: Float>(t39997: F, t118: F, t2001: F, t2281: F, t495: F, t305: F, t321: F, t2286: F, t34881: F, t16156: F, t9051: F, t36343: F, t9147: F) -> (F, F, F, F, F, F) {
    let t39998 = F::new(0.15965655602485078085e0) * t39997;
    let t40001 = t2001 * t118 * t2281 * t495;
    let t40031 = t2001 * t305 * t2281 * t321;
    let t40045 = t34881 * t2286;
    let t40062 = t16156 * t9051;
    let t40063 = F::new(0.19863479950205658386e-4) * t40062;
    let t40075 = t36343 * t9147;
    (t39998, t40001, t40031, t40045, t40063, t40075)
}
