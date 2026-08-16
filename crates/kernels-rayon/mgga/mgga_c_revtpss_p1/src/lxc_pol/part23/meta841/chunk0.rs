//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2716/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2716(t1261: f64, t12879: f64, t247: f64, t6425: f64, t12772: f64, t21227: f64, t3625: f64, t21021: f64, t21007: f64, t44425: f64, t21222: f64, t5340: f64) -> (f64, f64, f64, f64, f64) {
    let t70032 = t1261 * t247 * t12879 * t6425;
    let t70039 = t3625 * t12772 * t21227;
    let t70044 = t3625 * t12772 * t21021;
    let t70064 = t3625 * t44425 * t21007;
    let t70076 = t5340 * t12772 * t21222;
    (t70032, t70039, t70044, t70064, t70076)
}
