//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1118/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1118<F: Float>(t108210: F, t192: F, t2506: F, t24543: F, t27802: F, t2: F, t27742: F, t27816: F, t96925: F, t27750: F, t1434: F, t2399: F, t6887: F, t6884: F, t96982: F, t6899: F, t89: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t108211 = 2.0 / 9.0 * t108210;
    let t108212 = t192 * t2506;
    let t108249 = t24543 * t27802;
    let t108250 = t108249 / 9.0;
    let t108255 = t2 * t27742;
    let t108260 = t96925 * t27816;
    let t108261 = t108260 / 3.0;
    let t108262 = t24543 * t27750;
    let t108263 = 2.0 * t108262;
    let t108278 = t1434 * t2399 * t6887;
    let t108284 = t96982 * t6884;
    let t108291 = t89 * t2399 * t6899;
    (t108211, t108212, t108249, t108250, t108255, t108260, t108261, t108262, t108263, t108278, t108284, t108291)
}
