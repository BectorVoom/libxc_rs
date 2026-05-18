//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 944/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk944<F: Float>(t141203: F, t33485: F, t375: F, t89: F, t33465: F, t681: F, t33469: F, t33288: F, t33308: F, t7511: F, t33303: F, t33333: F, t6109: F) -> (F, F, F, F, F, F, F) {
    let t141204 = F::new(4.0) / F::new(9.0) * t141203;
    let t141206 = t89 * t375 * t33485;
    let t141220 = t89 * t681 * t33465;
    let t141223 = t89 * t681 * t33469;
    let t141231 = t7511 * t33288 * t33308;
    let t141240 = t7511 * t33288 * t33303;
    let t141255 = t6109 * t681 * t33333;
    (t141204, t141206, t141220, t141223, t141231, t141240, t141255)
}
