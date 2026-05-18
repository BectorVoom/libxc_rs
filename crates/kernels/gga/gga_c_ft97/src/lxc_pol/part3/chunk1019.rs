//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 1019/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk1019<F: Float>(t10658: F, t19273: F, t19276: F, t19278: F, t19283: F, t19287: F, t19292: F, t19295: F, t19298: F, t19301: F, t19304: F, t19737: F, t19748: F, t19769: F) -> F {
    let t19780 = F::new(2.0) / F::new(27.0) * t19273 + F::new(4.0) / F::new(9.0) * t19276 - F::new(2.0) / F::new(27.0) * t19278 - t10658 + F::new(2.0) / F::new(3.0) * t19283 - t19287 / F::new(9.0) - F::new(2.0) * t19292 + F::new(4.0) / F::new(3.0) * t19295 + t19298 / F::new(27.0) - F::new(2.0) / F::new(27.0) * t19301 + F::new(2.0) / F::new(81.0) * t19304;
    let t19782 = t19737 + t19748 + t19769 + t19780;
    t19782
}
