//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 835/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk835<F: Float>(t33350: F, t695: F, t141116: F, t2387: F, t2917: F, t36791: F, t108517: F, t141111: F, t1636: F, t7528: F, t89: F, t7532: F, t33485: F, t375: F, t33465: F, t681: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t141166 = t33350 * t695;
    let t141171 = t2387 * t141116;
    let t141172 = t36791 * t2917;
    let t141176 = t108517 * t141111;
    let t141200 = t89 * t1636 * t7528;
    let t141201 = 8.0 / 9.0 * t141200;
    let t141203 = t89 * t1636 * t7532;
    let t141204 = 4.0 / 9.0 * t141203;
    let t141206 = t89 * t375 * t33485;
    let t141220 = t89 * t681 * t33465;
    (t141166, t141171, t141172, t141176, t141200, t141201, t141203, t141204, t141206, t141220)
}
