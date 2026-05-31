//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1036/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1036<F: Float>(t57435: F, t73256: F, t73259: F, t73262: F, t73276: F, t73299: F, t73301: F, t86016: F, t86020: F, t86172: F, t86175: F, t86178: F, t86181: F, t86188: F, t86195: F) -> F {
    let t86354 = -F::cast_from(5.0_f64) / F::cast_from(16.0_f64) * t86016 - t86020 / F::cast_from(4.0_f64) + t86172 / F::cast_from(6.0_f64) - F::cast_from(4.0_f64) * t86175 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t86178 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t86181 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t73256 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t73259 - F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t73262 + F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t73276 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t86188 + F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t57435 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t73299 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t73301 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t86195;
    t86354
}
