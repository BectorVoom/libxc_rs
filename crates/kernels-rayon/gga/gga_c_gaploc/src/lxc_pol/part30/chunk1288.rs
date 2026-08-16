//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1288/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1288(t32893: f64, t7427: f64, t7573: f64, t10892: f64, t1980: f64, t11026: f64, t5782: f64, t11030: f64, t2365: f64, t24741: f64, t6111: f64, t32514: f64, t6066: f64, t7630: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33205 = 0.62115540045351614476e2_f64 * t7427 * t7573 * t32893;
    let t33206 = t1980 * t10892;
    let t33210 = 0.13803453343411469884e2_f64 * t5782 * t11026;
    let t33212 = 0.13803453343411469884e2_f64 * t5782 * t11030;
    let t33214 = t6111 * t2365 * t24741;
    let t33215 = 0.59584149919750711116e-1_f64 * t33214;
    let t33218 = 0.85801175884441024006e1_f64 * t7630 * t6066 * t32514;
    (t33205, t33206, t33210, t33212, t33215, t33218)
}
