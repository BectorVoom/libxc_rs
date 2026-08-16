//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3245/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3245(t10845: f64, t18531: f64, t18618: f64, t2741: f64, t18622: f64, t14785: f64, t18627: f64, t2745: f64, t2747: f64, t2749: f64, t2754: f64, t50351: f64, t5962: f64, t61550: f64, t61560: f64, t61564: f64, t61568: f64, t61570: f64, t836: f64) -> f64 {
    let t61572 = t10845 * t18531;
    let t61574 = t2741 * t18618;
    let t61576 = t10845 * t18622;
    let t61578 = 0.85748036236139473944e-3_f64 * t2745 * t2747 * t18627 * t2754 - 0.80031500487063509015e-2_f64 * t61550 + 0.2032800112371413129e-3_f64 * t50351 - 0.85748036236139473944e-2_f64 * t2745 * t14785 * t5962 * t836 * t2749 - 0.28582678745379824648e-4_f64 * t61560 - 0.57165357490759649296e-4_f64 * t61564 - 0.57165357490759649296e-4_f64 * t61568 - 0.56688979511669985553e-2_f64 * t61570 + 0.13552000749142754193e-3_f64 * t61572 + 0.20007875121765877254e-2_f64 * t61574 + 0.13552000749142754193e-3_f64 * t61576;
    t61578
}
