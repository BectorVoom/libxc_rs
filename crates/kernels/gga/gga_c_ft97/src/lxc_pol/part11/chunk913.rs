//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 913/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk913<F: Float>(t37303: F, t37308: F, t37313: F, t37317: F, t37322: F, t37326: F, t37328: F, t37330: F, t37332: F, t37334: F, t37336: F, t37340: F, t37343: F, t37347: F, t37360: F) -> F {
    let t38792 = F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t37303 + F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t37308 - F::cast_from(20.0_f64) / F::cast_from(27.0_f64) * t37313 - F::cast_from(4.0_f64) * t37317 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t37322 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t37326 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t37328 + F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t37330 - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t37332 + F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t37334 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t37336 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t37340 - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t37343 - F::cast_from(16.0_f64) / F::cast_from(81.0_f64) * t37347 - F::cast_from(80.0_f64) / F::cast_from(243.0_f64) * t37360;
    t38792
}
