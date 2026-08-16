//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3201/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3201(t4241: f64, t21661: f64, t602: f64, t2246: f64, t5812: f64, t10309: f64, t13269: f64, t13272: f64, t13286: f64, t13289: f64, t13420: f64, t1497: f64, t21663: f64, t21809: f64, t2242: f64, t2247: f64, t2248: f64, t2315: f64, t4173: f64, t4178: f64, t5872: f64, t60221: f64, t60248: f64, t644: f64) -> f64 {
    let t60667 = t4241 * t4241;
    let t60670 = t21661 * t602;
    let t60673 = t5812 * t2246;
    let t60692 = -120.0_f64 * t10309 * t2248 * t5872 + 40.0_f64 * t21809 * t2247 * t644 - 16.0_f64 * t13269 * t4241 + 80.0_f64 * t13272 * t13286 + 40.0_f64 * t13272 * t13289 - 8.0_f64 * t13420 * t4173 - 8.0_f64 * t1497 * t60248 - 4.0_f64 * t21663 * t2315 - 8.0_f64 * t21809 * t2242 + 40.0_f64 * t2247 * t60667 + 20.0_f64 * t2248 * t60673 + 80.0_f64 * t4178 * t60221 - 8.0_f64 * t60670 * t644;
    t60692
}
