//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1224/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1224(t13: f64, t20226: f64, t2969: f64, t191: f64, t20229: f64, t25: f64, t2212: f64, t6452: f64, t8498: f64, t8514: f64, t8526: f64, t2026: f64, t3138: f64, t3142: f64, t800: f64) -> (f64, f64, f64, f64, f64) {
    let t23923 = t20226 * t13 * t2969;
    let t23925 = t25 * t20229 * t191;
    let t23930 = t2212 * t6452 * t191;
    let t23938 = t8526 * t8498 * t8514;
    let t23943 = t3138 * t800 * t2026 * t191 * t3142;
    (t23923, t23925, t23930, t23938, t23943)
}
