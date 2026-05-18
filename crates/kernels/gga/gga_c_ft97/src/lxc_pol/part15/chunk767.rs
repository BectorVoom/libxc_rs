//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 767/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk767<F: Float>(t216: F, t5011: F, t207: F, t1690: F, t5010: F, t1096: F, t5025: F, t680: F, t1127: F, t4960: F, t2394: F, t1111: F, t17987: F, t2035: F, t21172: F, t21224: F, t21225: F, t21227: F, t21239: F, t21243: F, t21250: F, t21253: F, t238: F, t2387: F, t3766: F, t3767: F, t3789: F, t3790: F, t4987: F, t5049: F, t9533: F) -> (F, F, F, F) {
    let t21260 = t5011 * t216;
    let t21262 = F::new(1.0) / t207 / t21260;
    let t21264 = t1690 * t5010 * t21262;
    let t21268 = t680 * t1096 * t5025;
    let t21271 = t4960 * t1127;
    let t21272 = t2394 * t21271;
    let t21275 = -F::new(0.42160609613301514757e-3) * t17987 * t2035 * t21172 - t21224 + F::new(6.0) * t21225 - F::new(6.0) * t21227 + F::new(2.0) * t21239 + F::new(0.35564283887055077925e-1) * t4987 * t1111 - F::new(6.0) * t3766 * t21243 - F::new(6.0) * t3766 * t3767 * t5049 + F::new(0.10261957230907473486e-6) * t3789 * t21250 * t21253 + F::new(6.0) * t3789 * t3790 * t5049 - F::new(0.26701719421757626014e-2) * t238 * t21264 - F::new(0.69764702839313376e-1) * t9533 * t21268 - F::new(0.69764702839313376e-2) * t2387 * t21272;
    (t21262, t21264, t21271, t21275)
}
