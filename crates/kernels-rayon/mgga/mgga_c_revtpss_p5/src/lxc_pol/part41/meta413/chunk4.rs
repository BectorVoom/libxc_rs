//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1459/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1459(t4147: f64, t6781: f64, t4140: f64, t6836: f64, t1353: f64, t13615: f64, t13620: f64, t13623: f64, t13634: f64, t13635: f64, t22187: f64, t22189: f64, t22192: f64, t22194: f64, t22196: f64, t22197: f64, t22198: f64, t22199: f64, t22200: f64, t22201: f64, t22202: f64, t4139: f64, t5536: f64, t9394: f64, t9415: f64) -> f64 {
    let t22466 = t6781 * t4147;
    let t22470 = t4140 * t6836;
    let t22473 = -3.0_f64 * t1353 * t22466 * t4139 + 6.0_f64 * t22470 * t5536 - t13615 - t13620 - t13623 + t13634 - t13635 - t22187 + t22189 - t22192 + t22194 + t22196 - t22197 - t22198 - t22199 - t22200 + t22201 + t22202 + t9394 - t9415;
    t22473
}
