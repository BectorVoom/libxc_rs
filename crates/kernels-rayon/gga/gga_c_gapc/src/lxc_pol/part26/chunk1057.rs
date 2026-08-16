//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 1057/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk1057(t11725: f64, t2469: f64, t2470: f64, t24915: f64, t2822: f64, t3268: f64, t33103: f64, t33105: f64, t33110: f64, t33113: f64, t33114: f64, t33116: f64, t33121: f64, t33129: f64, t3746: f64, t3795: f64, t7053: f64, t7056: f64, t7063: f64, t972: f64) -> f64 {
    let t33137 = 2.0_f64 * t2469 * t2822 * t3795 - 6.0_f64 * t2822 * t3746 * t7063 + 4.0_f64 * t11725 * t7056 + 2.0_f64 * t2470 * t33129 + 8.0_f64 * t24915 * t3268 - 2.0_f64 * t33121 * t972 - t3795 * t7053 - t33103 + t33105 + t33110 + t33113 + t33114 - t33116;
    t33137
}
