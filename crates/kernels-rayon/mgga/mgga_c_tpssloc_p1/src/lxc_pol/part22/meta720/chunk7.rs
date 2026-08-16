//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2341/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2341(t20944: f64, t41011: f64, t119: f64, t13365: f64, t1516: f64, t16976: f64, t20943: f64, t210: f64, t2571: f64, t41084: f64, t41161: f64, t4119: f64, t4158: f64, t4261: f64, t46887: f64, t46912: f64, t46929: f64, t5544: f64, t5567: f64, t5624: f64, t58744: f64, t58834: f64, t67282: f64, t776: f64, t787: f64, t820: f64, t843: f64, t847: f64, t9559: f64) -> f64 {
    let t67937 = t41011 * t20944;
    let t67957 = -t843 * t847 * t820 * t67282 / 768.0_f64 + 5.0_f64 / 256.0_f64 * t13365 * t5624 - t58834 * t1516 / 256.0_f64 - t16976 * t4261 / 256.0_f64 + t46887 + 7.0_f64 / 12.0_f64 * t67937 - t787 * t210 * t119 * t67282 / 48.0_f64 + t46912 + 35.0_f64 / 24.0_f64 * t58744 + 455.0_f64 / 648.0_f64 * t41084 - t46929 + 5.0_f64 / 4.0_f64 * t41161 * t210 * t20943 * t776 - 3.0_f64 / 4.0_f64 * t9559 * t210 * t5567 * t4119 + 3.0_f64 / 16.0_f64 * t2571 * t210 * t4158 * t5544;
    t67957
}
