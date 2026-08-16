//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 767/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk767(t216: f64, t5011: f64, t207: f64, t1690: f64, t5010: f64, t1096: f64, t5025: f64, t680: f64, t1127: f64, t4960: f64, t2394: f64, t1111: f64, t17987: f64, t2035: f64, t21172: f64, t21224: f64, t21225: f64, t21227: f64, t21239: f64, t21243: f64, t21250: f64, t21253: f64, t238: f64, t2387: f64, t3766: f64, t3767: f64, t3789: f64, t3790: f64, t4987: f64, t5049: f64, t9533: f64) -> (f64, f64, f64, f64) {
    let t21260 = t5011 * t216;
    let t21262 = 1.0_f64 / t207 / t21260;
    let t21264 = t1690 * t5010 * t21262;
    let t21268 = t680 * t1096 * t5025;
    let t21271 = t4960 * t1127;
    let t21272 = t2394 * t21271;
    let t21275 = -0.42160609613301514757e-3_f64 * t17987 * t2035 * t21172 - t21224 + 6.0_f64 * t21225 - 6.0_f64 * t21227 + 2.0_f64 * t21239 + 0.35564283887055077925e-1_f64 * t4987 * t1111 - 6.0_f64 * t3766 * t21243 - 6.0_f64 * t3766 * t3767 * t5049 + 0.10261957230907473486e-6_f64 * t3789 * t21250 * t21253 + 6.0_f64 * t3789 * t3790 * t5049 - 0.26701719421757626014e-2_f64 * t238 * t21264 - 0.69764702839313376e-1_f64 * t9533 * t21268 - 0.69764702839313376e-2_f64 * t2387 * t21272;
    (t21262, t21264, t21271, t21275)
}
