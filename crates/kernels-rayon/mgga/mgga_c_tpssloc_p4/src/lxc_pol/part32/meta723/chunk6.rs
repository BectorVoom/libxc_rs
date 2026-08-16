//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2312/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2312(t1186: f64, t11881: f64, t1751: f64, t19145: f64, t19165: f64, t24812: f64, t24814: f64, t24815: f64, t27517: f64, t27533: f64, t27549: f64, t27550: f64, t29708: f64, t29711: f64, t29719: f64, t29726: f64, t3242: f64, t3610: f64, t3624: f64, t3961: f64, t5068: f64, t5079: f64, t6146: f64, t7283: f64, t7381: f64, t94395: f64, t95092: f64, t95163: f64, t95165: f64, t95192: f64, t95213: f64) -> f64 {
    let t103918 = -2.0_f64 * t3624 * t29719 * t5079 + 2.0_f64 * t3610 * t29711 * t5068 - 0.82246703342411321825e-2_f64 * t7283 * t6146 * t7381 + 0.14621636149762012769e-1_f64 * t95092 * t27533 - t95163 - 0.14621636149762012769e-1_f64 * t94395 * t27517 + 0.16449340668482264365e-1_f64 * t24812 * t24814 * t19145 * t24815 + 0.73108180748810063845e-2_f64 * t27549 * t27550 * t1751 * t3242 * t3961 + t95165 - t95192 + t95213 + 6.0_f64 * t11881 * t29708 * t19165 - 0.82246703342411321825e-2_f64 * t7283 * t1186 * t29726;
    t103918
}
