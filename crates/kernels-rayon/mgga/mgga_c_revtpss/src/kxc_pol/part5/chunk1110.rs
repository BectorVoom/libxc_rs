//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1110/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1110(t16088: f64, t16094: f64, t3169: f64, t4820: f64, t3188: f64, t4817: f64, t1065: f64, t4772: f64, t247: f64, t3109: f64, t4583: f64, t1063: f64) -> (f64, f64, f64, f64, f64) {
    let t16095 = t16094 * t16088;
    let t16121 = 0.15244095330869239812e-2_f64 * t3169 * t4820;
    let t16134 = 0.19055119163586549765e-3_f64 * t3188 * t4817;
    let t16138 = t1065 * t4772;
    let t16158 = t247 * t3109 * t4583;
    let t16160 = 0.19055119163586549765e-3_f64 * t1063 * t16158;
    (t16095, t16121, t16134, t16138, t16160)
}
