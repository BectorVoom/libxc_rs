//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 861/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk861(t16617: f64, t12943: f64, t16630: f64, t12946: f64, t145: f64, t20741: f64, t185: f64, t4315: f64, t5544: f64, t1484: f64, t16606: f64, t193: f64, t20753: f64, t20756: f64, t2522: f64, t262: f64, t4314: f64, t9780: f64, t9789: f64, t9793: f64, t9797: f64, t9863: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20760 = 0.17544670867903938621e1_f64 * t16617;
    let t20761 = 0.35089341735807877242e1_f64 * t12943;
    let t20765 = 24.0_f64 * t16630;
    let t20766 = 12.0_f64 * t12946;
    let t20767 = t145 * t20741;
    let t20768 = t20767 * t185;
    let t20769 = t4315 * t5544;
    let t20772 = 9.0_f64 * t1484 * t16606 * t2522 + 6.0_f64 * t193 * t20756 * t262 + 18.0_f64 * t20753 * t4314 + 18.0_f64 * t20769 * t4314 - t20760 + t20761 + t20765 + t20766 + t20768 + t9780 - t9789 + t9793 + t9797 + t9863;
    (t20760, t20761, t20765, t20766, t20768, t20772)
}
