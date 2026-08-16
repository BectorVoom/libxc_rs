//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1065/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1065<F: Float>(t41448: F, t9920: F, t2497: F, t41468: F, t2503: F, t8282: F, t2489: F, t9953: F, t2: F, t41446: F, t1775: F, t9922: F) -> (F, F, F, F, F, F, F) {
    let t42071 = t9920 * t41448;
    let t42075 = t2497 * t41468;
    let t42079 = t8282 * t2503;
    let t42081 = t8282 * t2489;
    let t42083 = t9953 * t41448;
    let t42087 = t2 * t41446;
    let t42088 = t42087 * t41448;
    let t42092 = t1775 * t9922;
    (t42071, t42075, t42079, t42081, t42083, t42088, t42092)
}
