//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2225/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2225(t13269: f64, t1470: f64, t4173: f64, t4181: f64, t4187: f64, t21698: f64, t603: f64, t101326: f64, t1928: f64, t28105: f64, t28109: f64, t28112: f64, t28116: f64, t28119: f64, t28138: f64, t29554: f64, t6974: f64, t6978: f64, t7706: f64, t7716: f64) -> f64 {
    let t108807 = t13269 * t1470;
    let t108810 = t4173 * t4181;
    let t108813 = t4173 * t4187;
    let t108816 = t603 * t21698;
    let t108829 = 5.0_f64 / 3.0_f64 * t101326 * t7706 + 5.0_f64 / 3.0_f64 * t28138 * t28105 + 5.0_f64 / 3.0_f64 * t28138 * t28109 + 2.0_f64 / 3.0_f64 * t108807 * t1928 + 2.0_f64 / 3.0_f64 * t108810 * t1928 + 2.0_f64 / 3.0_f64 * t108813 * t1928 + t108816 * t1928 / 3.0_f64 + t29554 * t6974 / 3.0_f64 + t29554 * t6978 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t28112 * t7716 + 2.0_f64 / 3.0_f64 * t28116 * t7716 + 2.0_f64 / 3.0_f64 * t28119 * t7716;
    t108829
}
