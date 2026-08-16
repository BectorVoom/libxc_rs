//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1289/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1289(t3169: f64, t4820: f64, t1015: f64, t13312: f64, t1012: f64, t16096: f64, t4573: f64, t11703: f64, t3188: f64, t4817: f64, t1011: f64, t11268: f64, t11714: f64, t11967: f64, t11972: f64, t11980: f64, t11989: f64, t12007: f64, t12010: f64, t16095: f64, t1671: f64, t1675: f64) -> f64 {
    let t16121 = 0.15244095330869239812e-2_f64 * t3169 * t4820;
    let t16122 = t1015 * t13312;
    let t16123 = t1012 * t16122;
    let t16127 = t4573 * t16096;
    let t16128 = t11703 * t16127;
    let t16134 = 0.19055119163586549765e-3_f64 * t3188 * t4817;
    let t16136 = 0.5081365110289746604e-3_f64 * t11967 + t11972 + 0.28582678745379824648e-3_f64 * t11980 - 0.63517063878621832551e-4_f64 * t11989 + 0.72409452821628889107e-2_f64 * t11268 * t1671 - t16121 + t1011 * t16123 / 288.0_f64 - 0.10162730220579493208e-2_f64 * t12007 - 0.47637797908966374414e-3_f64 * t16095 * t16128 - 0.15244095330869239812e-2_f64 * t11714 * t1675 + t16134 + 0.28582678745379824648e-3_f64 * t12010;
    t16136
}
