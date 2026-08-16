//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1038/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1038<F: Float>(t38771: F, t57620: F, t73343: F, t73358: F, t73405: F, t86246: F, t86250: F, t86254: F, t86258: F, t86264: F, t86268: F, t86274: F, t86278: F, t86281: F, t86284: F) -> F {
    let t86386 = F::cast_from(40.0_f64) / F::cast_from(27.0_f64) * t86246 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t86250 - F::cast_from(80.0_f64) / F::cast_from(243.0_f64) * t86254 - t86258 / F::cast_from(9.0_f64) - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t73343 + t38771 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t73358 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t86264 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t86268 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t57620 - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t73405 + F::cast_from(8.0_f64) * t86274 + F::cast_from(2.0_f64) * t86278 + F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t86281 - t86284 / F::cast_from(3.0_f64);
    t86386
}
