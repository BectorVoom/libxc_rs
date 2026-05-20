//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 984/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk984<F: Float>(t128: F, t121: F, t22: F, t2508: F, t9285: F, t692: F, t9288: F, t124: F, t624: F, t138: F) -> (F, F, F, F) {
    let t9294 = F::new(1.0)/pow_3_2::<F>(t128);
    let t9295 = t9294 * t121;
    let t9296 = t9295 * t22;
    let t9298 = t2508 * t9285;
    let t9300 = t692 * t9288;
    let t9302 = t124 * t624;
    let t9303 = t138 * t9302;
    (t9296, t9298, t9300, t9303)
}
