//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1165/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1165<F: Float>(t56986: F, t57007: F, t57052: F, t57108: F, t779: F, t799: F, t4818: F, t24302: F, t24305: F, t16817: F, t3793: F, t845: F, t39411: F, t39413: F, t39418: F, t49240: F, t49242: F, t49271: F, t49273: F, t56966: F, t56969: F, t56972: F, t56975: F, t56978: F, t56981: F, t56984: F) -> (F, F, F, F, F) {
    let t57113 = 1.0 * t779 * (t56986 + t57007 + t57052 + t57108) * t799;
    let t57114 = t4818 * t4818;
    let t57117 = 0.24954977986735470917e5 * t24302 * t57114 * t24305;
    let t57120 = 0.46785787179641632568e1 * t845 * t3793 * t16817;
    let t57135 = -0.80513333333333333336e0 * t39411 - 0.53675555555555555556e0 * t39413 + 0.16102666666666666667e1 * t39418 + 0.80513333333333333333e0 * t49240 - 0.24154e1 * t49242 - 0.132456e1 * t49271 + 0.22076e0 * t49273 + 0.72462e1 * t56966 - 0.20128333333333333334e1 * t56969 - 0.11038e0 * t56972 - 0.22076e0 * t56975 - 0.108693e2 * t56978 + 0.24154e1 * t56981 - 0.80513333333333333332e0 * t56984;
    (t57113, t57114, t57117, t57120, t57135)
}
