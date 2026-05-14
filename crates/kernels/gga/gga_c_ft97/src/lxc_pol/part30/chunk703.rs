//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 703/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk703<F: Float>(t245: F, t33534: F, t33799: F, t21: F, t363: F, t5: F, t7565: F, t7580: F, t92: F) -> (F, F, F) {
    let t246 = 10000000.0 <= t245;
    let t33800 = t33534 + t33799;
    let t33807 = piecewise3(t246, 0.0, t5 * t33800 * t21 / 4.0 + t5 * t7565 * t363 / 4.0);
    let t33808 = t7580 * t92;
    (t33800, t33807, t33808)
}
