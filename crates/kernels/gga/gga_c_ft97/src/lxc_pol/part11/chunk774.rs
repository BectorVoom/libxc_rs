//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 774/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk774<F: Float>(t1611: F, t533: F, t37454: F, t384: F, t7977: F, t7924: F, t8002: F, t1608: F, t7905: F, t8007: F, t1620: F, t6: F, t7900: F, t8010: F, t8018: F, t7837: F, t8008: F) -> (F, F, F, F, F, F, F) {
    let t37487 = t1611 * t533;
    let t37488 = t37487 * t37454;
    let t37495 = t384 * t7977;
    let t37499 = t8002 * t7924;
    let t37504 = t1608 * t8007 * t7905;
    let t37506 = t7900 * t6 * t1620;
    let t37509 = t8010 * t8018;
    let t37518 = t7837 * t8008;
    (t37488, t37495, t37499, t37504, t37506, t37509, t37518)
}
