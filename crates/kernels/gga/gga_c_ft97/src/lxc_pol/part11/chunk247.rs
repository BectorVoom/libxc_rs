//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 247/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk247<F: Float>(t238: F, t695: F, t709: F, t224: F, t678: F, t690: F) -> F {
    let t239 = F::new(0.1e-59) < t238;
    let t710 = t695 * t709;
    let t713 = piecewise3::<F>(t239, -F::cast_from(0.11627450473218896e-1_f64) * t678 * t690 - t224 * t710, F::new(0.0));
    t713
}
