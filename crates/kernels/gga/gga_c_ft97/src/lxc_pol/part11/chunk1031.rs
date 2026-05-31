//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1031/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1031<F: Float>(t173: F, t701: F, t9673: F, t191: F, t2347: F, t2346: F) -> (F, F, F) {
    let t41531 = t701 * t173 * t9673;
    let t41534 = F::cast_from(1.0_f64) / t191 / t2347;
    let t41535 = t2346 * t2346;
    let t41536 = F::cast_from(1.0_f64) / t41535;
    (t41531, t41534, t41536)
}
