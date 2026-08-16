//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1032/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1032(t6157: f64, t954: f64, t2950: f64, t2957: f64, t4571: f64, t4620: f64, t6094: f64, t6098: f64, t6102: f64, t6114: f64, t6121: f64, t6127: f64, t6129: f64, t6133: f64, t6136: f64, t6139: f64) -> (f64, f64) {
    let t6158 = t6157 * t954;
    let t6173 = -0.17648625e1_f64 * t6114 + 0.3529725e1_f64 * t6121 + t2950 + 0.34431666666666666666e0_f64 * t4571 - 0.34431666666666666667e0_f64 * t6094 + 0.103295e1_f64 * t6098 - 0.516475e0_f64 * t6102 + 0.31558125e0_f64 * t6127 + 0.6311625e0_f64 * t6129 + t2957 + 0.13892666666666666667e0_f64 * t4620 - 0.34731666666666666667e-1_f64 * t6133 + 0.20839e0_f64 * t6136 - 0.104195e0_f64 * t6139;
    (t6158, t6173)
}
