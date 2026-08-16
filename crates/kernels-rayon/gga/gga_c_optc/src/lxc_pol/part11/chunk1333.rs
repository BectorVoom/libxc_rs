//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1333/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1333(t39411: f64, t39413: f64, t39418: f64, t49240: f64, t49242: f64, t49271: f64, t49273: f64, t56966: f64, t56969: f64, t56972: f64, t56975: f64, t23683: f64, t23686: f64, t30189: f64, t30270: f64, t49378: f64, t49381: f64, t56978: f64, t56981: f64, t56984: f64, t56988: f64, t56991: f64, t56994: f64) -> (f64, f64) {
    let t58056 = -0.51702222222222222221e1_f64 * t39411 - 0.34468148148148148146e1_f64 * t39413 + 0.10340444444444444444e2_f64 * t39418 + 0.5170222222222222222e1_f64 * t49240 - 0.15510666666666666667e2_f64 * t49242 - 0.19704e-1_f64 * t49271 + 0.3284e-2_f64 * t49273 + 0.46531999999999999998e2_f64 * t56966 - 0.12925555555555555555e2_f64 * t56969 - 0.1642e-2_f64 * t56972 - 0.3284e-2_f64 * t56975;
    let t58067 = -0.69798e2_f64 * t56978 + 0.15510666666666666667e2_f64 * t56981 - 0.5170222222222222222e1_f64 * t56984 - 0.44334e-1_f64 * t56988 + 0.9852e-2_f64 * t56991 + 0.14778e-1_f64 * t56994 + 0.14595555555555555556e-1_f64 * t30189 + t23683 + t23686 + 0.14595555555555555556e-2_f64 * t49378 + 0.3284e-2_f64 * t49381 + 0.8042567901234567901e1_f64 * t30270;
    (t58056, t58067)
}
