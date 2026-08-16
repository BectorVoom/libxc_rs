//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1352/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1352(t16892: f64, t16708: f64, t16710: f64, t16717: f64, t16722: f64, t16735: f64, t16740: f64, t16744: f64, t16908: f64, t16927: f64, t16931: f64, t16933: f64) -> (f64, f64) {
    let t17066 = 0.27785333333333333334e0_f64 * t16892;
    let t17075 = 0.22954444444444444444e0_f64 * t16708;
    let t17083 = 0.46308888888888888889e-1_f64 * t16908 + 0.6311625e0_f64 * t16927 - 0.68863333333333333333e0_f64 * t16710 + t17075 + 0.46308888888888888889e-1_f64 * t16931 + 0.3529725e1_f64 * t16933 - 0.20659e1_f64 * t16722 + 0.20659e1_f64 * t16740 + 0.103295e1_f64 * t16744 + 0.309885e1_f64 * t16735 + 0.57386111111111111112e0_f64 * t16717;
    (t17066, t17083)
}
