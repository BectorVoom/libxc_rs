//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 721/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk721<F: Float>(t33253: F, t6009: F, t193: F, t1425: F, t6192: F, t6154: F, t6187: F, t1454: F, t6062: F, t1449: F, t24429: F, t7536: F, t761: F) -> (F, F, F, F, F, F, F, F, F) {
    let t33254 = t33253 * t6009;
    let t33255 = t193 * t33254;
    let t33258 = t1425 * t6192;
    let t33259 = t193 * t33258;
    let t33264 = t6154 * t6187;
    let t33268 = t6062 * t1454;
    let t33269 = t193 * t33268;
    let t33272 = t24429 * t1449;
    let t33274 = t7536 * t761;
    (t33254, t33255, t33258, t33259, t33264, t33268, t33269, t33272, t33274)
}
