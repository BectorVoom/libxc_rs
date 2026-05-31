//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1044/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1044<F: Float>(t57767: F, t59426: F, t59486: F, t73439: F, t73442: F, t74307: F, t74374: F, t74377: F, t86289: F, t86297: F, t86300: F, t86303: F, t86306: F, t86309: F) -> F {
    let t86477 = -t86289 / F::cast_from(6.0_f64) - t59426 + t57767 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t73439 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t73442 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t74307 + F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t74374 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t86297 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t86300 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t86303 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t86306 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t86309 + F::cast_from(20.0_f64) / F::cast_from(243.0_f64) * t74377 - t59486;
    t86477
}
