//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 928/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk928<F: Float>(t31538: F, t6243: F, t14: F, t2724: F, t6250: F, t19116: F, t287: F, t7005: F, t172: F, t231: F, t816: F, t37481: F, t40: F) -> (F, F, F, F, F) {
    let t127651 = t31538 * t6243;
    let t127654 = t2724 * t14;
    let t127655 = t127654 * t6250;
    let t127659 = t19116 * t287 * t7005;
    let t127680 = t816 * t172 * t231;
    let t136595 = t40 * t37481;
    (t127651, t127655, t127659, t127680, t136595)
}
