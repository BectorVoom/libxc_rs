//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1131/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1131(t1769: f64, t8832: f64, t1044: f64, t5389: f64, t3491: f64, t5165: f64, t639: f64, t9099: f64, t1676: f64, t192: f64, t8817: f64, t306: f64, t9539: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t24729 = t1769 * t8832;
    let t24792 = t5389 * t1044;
    let t24927 = t3491 * t5165;
    let t24934 = t9099 * t639;
    let t24941 = t9099 * t1676;
    let t24964 = t192 * t8817;
    let t25113 = t306 * t9539;
    (t24729, t24792, t24927, t24934, t24941, t24964, t25113)
}
