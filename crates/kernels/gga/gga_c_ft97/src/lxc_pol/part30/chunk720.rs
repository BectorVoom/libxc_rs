//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 720/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk720<F: Float>(t33243: F, t6009: F, t193: F, t24240: F, t6008: F, t2371: F, t7484: F) -> (F, F, F, F, F) {
    let t33244 = t33243 * t6009;
    let t33245 = t193 * t33244;
    let t33247 = t6008 * t24240;
    let t33248 = t193 * t33247;
    let t33253 = t2371 * t7484;
    (t33244, t33245, t33247, t33248, t33253)
}
