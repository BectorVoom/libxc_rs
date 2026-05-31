//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2133/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2133<F: Float>(t1937: F, t98487: F, t27123: F, t6993: F, t25803: F, t7898: F, t2033: F, t47672: F, t1907: F, t4144: F, t28196: F, t27833: F, t7313: F) -> (F, F, F, F, F) {
    let t98489 = F::cast_from(4.0_f64) * t98487 * t1937;
    let t98491 = F::cast_from(4.0_f64) * t27123 * t6993;
    let t98494 = t7898 * t25803;
    let t98495 = t2033 * t47672;
    let t98496 = t1907 * t4144;
    let t98499 = F::cast_from(6.0_f64) * t28196 * t98495 * t98496;
    let t98501 = F::cast_from(2.0_f64) * t27833 * t7313;
    (t98489, t98491, t98494, t98499, t98501)
}
