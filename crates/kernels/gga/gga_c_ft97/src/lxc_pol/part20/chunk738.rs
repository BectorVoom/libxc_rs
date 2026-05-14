//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 738/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk738<F: Float>(t24191: F, t6009: F, t193: F, t2354: F, t2413: F, t6003: F, t2405: F, t9744: F, t5995: F, t92: F) -> (F, F, F, F, F) {
    let t24192 = t24191 * t6009;
    let t24193 = t193 * t24192;
    let t24197 = t2354 * t6003 * t2413;
    let t24201 = t9744 * t6003 * t2405;
    let t24204 = t5995 * t92;
    (t24192, t24193, t24197, t24201, t24204)
}
