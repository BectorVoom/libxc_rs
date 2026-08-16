//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1272/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1272(t39411: f64, t39413: f64, t39418: f64, t49240: f64, t49242: f64, t49271: f64, t49273: f64, t56966: f64, t56969: f64, t56972: f64, t56975: f64, t56978: f64, t56981: f64, t56984: f64) -> f64 {
    let t56986 = -0.79724444444444444446e0_f64 * t39411 - 0.5314962962962962963e0_f64 * t39413 + 0.15944888888888888889e1_f64 * t39418 + 0.79724444444444444444e0_f64 * t49240 - 0.23917333333333333333e1_f64 * t49242 - 0.13145066666666666666e1_f64 * t49271 + 0.21908444444444444444e0_f64 * t49273 + 0.71752000000000000001e1_f64 * t56966 - 0.19931111111111111111e1_f64 * t56969 - 0.10954222222222222222e0_f64 * t56972 - 0.21908444444444444444e0_f64 * t56975 - 0.107628e2_f64 * t56978 + 0.23917333333333333333e1_f64 * t56981 - 0.79724444444444444444e0_f64 * t56984;
    t56986
}
