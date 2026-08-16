//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 516/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk516<F: Float>(t737: F, t762: F, t2486: F, t2492: F, t265: F, t9802: F, t1471: F, t4092: F, t2725: F, t6: F, t285: F, t1200: F) -> (F, F, F, F, F, F, F) {
    let t14182 = t737 * t762;
    let t14187 = t2486 * t762;
    let t14196 = t2492 * t265;
    let t14200 = t9802 * t265;
    let t14721 = t4092 * t1471;
    let t14728 = t2725 * t6;
    let t14729 = t285 * t14728;
    let t14742 = t1200 * t14728;
    (t14182, t14187, t14196, t14200, t14721, t14729, t14742)
}
