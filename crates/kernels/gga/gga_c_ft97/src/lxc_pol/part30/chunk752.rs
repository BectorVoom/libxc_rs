//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 752/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk752<F: Float>(t1096: F, t33356: F, t33357: F, t1418: F, t35415: F, t39: F, t6018: F, t3766: F, t22511: F, t6817: F, t213: F, t230: F, t420: F, t236: F, t5009: F, t21251: F, rho1: F) -> (F, F, F, F, F, F, F, F) {
    let t35446 = t33356 * t33357 * t1096;
    let t35449 = t1418 * t35415;
    let t35452 = t6018 * t39;
    let t35453 = t3766 * t35452;
    let t35454 = t22511 * t6817;
    let t35455 = t230 * t213;
    let t35456 = t420 * t35455;
    let t35457 = t35454 * t35456;
    let t35460 = t236 * t5009;
    let t35462 = 1.0 / t21251 / rho1;
    (t35446, t35449, t35453, t35454, t35455, t35457, t35460, t35462)
}
