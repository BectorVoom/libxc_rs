//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 893/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk893<F: Float>(t8326: F, t971: F, t8216: F, t1786: F, t3238: F, t3281: F, t981: F, t7943: F, t89: F, t973: F, t955: F, t951: F) -> (F, F, F, F, F, F, F) {
    let t47222 = t8326 * t971;
    let t47273 = t8216 * t971;
    let t47443 = t1786 * t3238;
    let t47727 = t3281 * t981;
    let t47836 = t89 * t7943 * t973;
    let t47860 = t3281 * t955;
    let t47926 = t3281 * t951;
    (t47222, t47273, t47443, t47727, t47836, t47860, t47926)
}
