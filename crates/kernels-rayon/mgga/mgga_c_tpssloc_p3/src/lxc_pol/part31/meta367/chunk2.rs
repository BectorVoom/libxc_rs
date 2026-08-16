//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1300/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1300(t9869: f64, t5519: f64, t706: f64, t708: f64, t9871: f64, t13115: f64, t157: f64, t4196: f64, t9880: f64, t13107: f64, t13105: f64, t9793: f64, t9797: f64, t9820: f64, t9824: f64, t9876: f64, t9884: f64, t9887: f64, t9890: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16688 = 4.0_f64 * t9869;
    let t16689 = t706 * t5519;
    let t16691 = 4.0_f64 * t16689 * t708;
    let t16692 = 0.24415263074675393405e-3_f64 * t9871;
    let t16693 = t13115 * t157;
    let t16695 = 24.0_f64 * t16693 * t4196;
    let t16696 = 0.10843581300301739842e-1_f64 * t9880;
    let t16697 = 0.34631718211362927517e2_f64 * t13107;
    let t16698 = t16688 + t16691 + t16692 + t9793 + t9797 - t9876 + t13105 - t9820 - t9824 + t16695 + t16696 - t9884 + t9887 + t9890 - t16697;
    (t16688, t16691, t16692, t16695, t16696, t16697, t16698)
}
