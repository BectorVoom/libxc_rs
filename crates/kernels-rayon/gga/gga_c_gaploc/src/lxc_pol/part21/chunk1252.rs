//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1252/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1252(t11030: f64, t5782: f64, t2365: f64, t24741: f64, t6111: f64, t32514: f64, t6066: f64, t7630: f64, t10827: f64, t825: f64, t826: f64, t3489: f64, t6100: f64) -> (f64, f64, f64, f64, f64) {
    let t33212 = 0.13803453343411469884e2_f64 * t5782 * t11030;
    let t33214 = t6111 * t2365 * t24741;
    let t33215 = 0.59584149919750711116e-1_f64 * t33214;
    let t33218 = 0.85801175884441024006e1_f64 * t7630 * t6066 * t32514;
    let t33220 = t825 * t826 * t10827;
    let t33221 = 0.51123901271894332902e0_f64 * t33220;
    let t33222 = t6100 * t3489;
    (t33212, t33215, t33218, t33221, t33222)
}
