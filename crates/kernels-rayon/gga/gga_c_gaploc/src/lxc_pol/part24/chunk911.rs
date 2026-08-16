//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 911/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk911(t6700: f64, t901: f64, t3162: f64, t549: f64, t1429: f64, t2372: f64, t2389: f64, t3248: f64, t731: f64, t3240: f64, t2549: f64, t7221: f64, t883: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9579 = 0.29792074959875355558e-1_f64 * t6700 * t901;
    let t9580 = t549 * t3162;
    let t9582 = 0.59584149919750711116e-1_f64 * t1429 * t9580;
    let t9584 = 0.59584149919750711116e-1_f64 * t2372 * t2389;
    let t9618 = 0.85450291446024714264e-3_f64 * t731 * t3248;
    let t9620 = 0.85450291446024714264e-3_f64 * t731 * t3240;
    let t9622 = 0.64087718584518535698e-3_f64 * t2549 * t3248;
    let t9624 = t883 * t7221;
    (t9579, t9580, t9582, t9584, t9618, t9620, t9622, t9624)
}
