//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 925/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk925<F: Float>(t123124: F, t27565: F, t109200: F, t17836: F, t6: F, t24389: F, t39: F, t108585: F, t17817: F, t27657: F, t3766: F, t22511: F, t33432: F, t3789: F) -> (F, F, F, F, F, F) {
    let t123125 = t123124 * t27565;
    let t123129 = t17836 * t109200 * t6;
    let t123133 = t17836 * t24389 * t39;
    let t123181 = t17817 * t108585;
    let t123408 = t3766 * t27657;
    let t123445 = t3789 * t33432 * t22511;
    (t123125, t123129, t123133, t123181, t123408, t123445)
}
