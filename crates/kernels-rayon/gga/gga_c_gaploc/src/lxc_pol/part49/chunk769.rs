//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 769/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk769(t3005: f64, t3295: f64, t9800: f64, t11053: f64, t9805: f64, t1029: f64, t9796: f64, t12665: f64, t12667: f64, t123: f64, t3431: f64, t883: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13052 = t3005 * t3295;
    let t13053 = t9800 * t13052;
    let t13054 = 0.19171462976960374838e1_f64 * t13053;
    let t13055 = t11053 * t3295;
    let t13056 = t9805 * t13055;
    let t13057 = 0.11502877786176224903e1_f64 * t13056;
    let t13058 = t1029 * t3295;
    let t13059 = t9796 * t13058;
    let t13060 = 0.76685851907841499353e0_f64 * t13059;
    let t13061 = 0.59584149919750711116e-1_f64 * t12665;
    let t13062 = 0.89376224879626066674e-1_f64 * t12667;
    let t13063 = t3431 * t123;
    let t13064 = t13063 * t883;
    (t13052, t13054, t13055, t13057, t13058, t13060, t13061, t13062, t13063, t13064)
}
