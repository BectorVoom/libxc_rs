//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 161/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk161<F: Float>(t79: F, t409: F, t938: F, t64: F, t372: F, t931: F) -> (F, F, F) {
    let t80 = F::cast_from(0.1e-59_f64) < t79;
    let t939 = t409 * t938;
    let t940 = t64 * t939;
    let t942 = piecewise3::<F>(t80, -F::cast_from(0.11627450473218896e-1_f64) * t372 * t931 - t940, F::cast_from(0.0_f64));
    (t939, t940, t942)
}
