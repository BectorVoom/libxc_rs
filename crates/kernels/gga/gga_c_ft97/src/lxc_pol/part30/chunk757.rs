//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 757/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk757<F: Float>(t193: F, t35537: F, t89: F, t35516: F, t676: F, t27: F, t33518: F, t33522: F, t33526: F, t35351: F, t35356: F, t35519: F, t35523: F, t35527: F, t35531: F, t35535: F) -> (F, F, F, F) {
    let t35538 = t193 * t35537;
    let t35539 = t89 * t35538;
    let t35541 = t676 * t35516;
    let t35543 = t89 * t27 * t35541;
    let t35545 = t33518 + t35351 / 18.0 + t35356 / 3.0 - t35519 / 6.0 - t33522 - 2.0 / 9.0 * t35523 - 2.0 * t35527 + 4.0 / 3.0 * t35531 + t33526 + t35535 / 9.0 + 2.0 / 3.0 * t35539 - t35543 / 3.0;
    (t35539, t35541, t35543, t35545)
}
