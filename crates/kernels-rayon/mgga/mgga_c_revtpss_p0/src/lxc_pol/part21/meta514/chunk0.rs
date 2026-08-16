//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2144/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2144(t16152: f64, t247: f64, t3116: f64, t3109: f64, t4583: f64, t1063: f64, t3172: f64, t4868: f64, t1041: f64, t2862: f64, t4823: f64, t1042: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16154 = t247 * t3116 * t16152;
    let t16158 = t247 * t3109 * t4583;
    let t16160 = 0.19055119163586549765e-3_f64 * t1063 * t16158;
    let t16163 = t3172 * t4868;
    let t16165 = 0.28582678745379824648e-3_f64 * t1041 * t16163;
    let t16166 = t4823 * t2862;
    let t16167 = t1042 * t16166;
    (t16154, t16158, t16160, t16163, t16165, t16166, t16167)
}
