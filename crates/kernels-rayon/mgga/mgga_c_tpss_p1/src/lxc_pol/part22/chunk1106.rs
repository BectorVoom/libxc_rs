//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1106/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1106(t1073: f64, t12083: f64, t12086: f64, t12136: f64, t12155: f64, t12159: f64, t12161: f64, t12164: f64, t12167: f64, t12170: f64, t1543: f64, t2932: f64, t2969: f64, t2976: f64, t4125: f64, t4147: f64, t4181: f64, t421: f64, t9365: f64, t9419: f64, t9471: f64) -> f64 {
    let t12175 = 0.5848223622634646207e0_f64 * t9365 * t1543 + 0.11696447245269292414e1_f64 * t2969 * t4181 - 2.0_f64 * t12083 * t2932 - 0.11696447245269292414e1_f64 * t12086 * t2976 + 0.5848223622634646207e0_f64 * t1073 * t12136 - 0.310907e-1_f64 * t12155 * t421 + t12159 - t12161 + t12164 + t12167 + t12170 - 4.0_f64 * t9471 * t4125 + 0.64327917994770140268e2_f64 * t9419 * t4147;
    t12175
}
