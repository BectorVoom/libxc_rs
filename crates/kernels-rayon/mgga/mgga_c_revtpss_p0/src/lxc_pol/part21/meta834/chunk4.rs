//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3126/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3126(t12772: f64, t17634: f64, t3625: f64, t17395: f64, t3746: f64, t1248: f64, t44586: f64, t17689: f64, t44425: f64, t17435: f64, t3667: f64, t1235: f64, t127: f64, t17278: f64, t371: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t57569 = t3625 * t12772 * t17634;
    let t57571 = t3746 * t17395;
    let t57578 = t44586 * t1248;
    let t57584 = t3625 * t44425 * t17689;
    let t57586 = t3667 * t17435;
    let t57590 = t1235 * t371 * t127 * t17278;
    (t57569, t57571, t57578, t57584, t57586, t57590)
}
