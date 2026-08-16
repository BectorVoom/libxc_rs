//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 742/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk742<F: Float>(t665: F, t679: F, t689: F, t39: F, t9681: F, t40: F, t3789: F) -> (F, F, F, F, F) {
    let t33427 = t665 * t679;
    let t33428 = t33427 * t689;
    let t33432 = t9681 * t39;
    let t33433 = t33432 * t40;
    let t33434 = t3789 * t33433;
    (t33427, t33428, t33432, t33433, t33434)
}
