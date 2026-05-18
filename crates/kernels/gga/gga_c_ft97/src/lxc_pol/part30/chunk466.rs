//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 466/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk466<F: Float>(t676: F, t7484: F, t27: F, t89: F, t7518: F, t7522: F, t7526: F, t7530: F) -> (F, F, F) {
    let t7532 = t676 * t7484;
    let t7534 = t89 * t27 * t7532;
    let t7536 = -t7518 / F::new(3.0) + t7522 / F::new(3.0) - t7526 / F::new(6.0) + F::new(2.0) / F::new(3.0) * t7530 - t7534 / F::new(3.0);
    (t7532, t7534, t7536)
}
