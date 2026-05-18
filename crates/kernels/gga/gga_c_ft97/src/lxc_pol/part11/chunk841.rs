//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 841/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk841<F: Float>(t37347: F, t37303: F, t37308: F, t37313: F, t37317: F, t37322: F, t37326: F, t37328: F, t37330: F, t37332: F, t37335: F, t37336: F, t37340: F, t37344: F) -> F {
    let t37348 = F::new(8.0) / F::new(81.0) * t37347;
    let t37349 = F::new(4.0) / F::new(9.0) * t37303 + F::new(20.0) / F::new(81.0) * t37308 - F::new(10.0) / F::new(27.0) * t37313 - F::new(2.0) * t37317 + F::new(4.0) / F::new(3.0) * t37322 + F::new(2.0) / F::new(9.0) * t37326 - F::new(4.0) / F::new(9.0) * t37328 + F::new(4.0) / F::new(27.0) * t37330 - F::new(4.0) / F::new(27.0) * t37332 + t37335 - F::new(2.0) / F::new(9.0) * t37336 + F::new(4.0) / F::new(3.0) * t37340 - t37344 - t37348;
    t37349
}
