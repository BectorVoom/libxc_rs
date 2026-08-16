//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 936/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk936(t3138: f64, t3142: f64, t8498: f64, t3139: f64, t763: f64, t2002: f64, t3141: f64, t13: f64, t2969: f64, t6449: f64) -> (f64, f64, f64, f64) {
    let t8501 = t3138 * t8498 * t3142 / 72.0_f64;
    let t8502 = t3139 * t763;
    let t8506 = t3141 * t2002;
    let t8511 = t6449 * t13 * t2969;
    (t8501, t8502, t8506, t8511)
}
