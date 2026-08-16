//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 310/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk310(t265: f64, t393: f64, t1079: f64, t1096: f64, t1000: f64, t1073: f64, t1076: f64, t342: f64, t386: f64, t989: f64, t995: f64, t389: f64, t198: f64, t336: f64, t895: f64, t912: f64, t938: f64, t978: f64, t980: f64, t985: f64) -> (f64, f64, f64, f64) {
    let t394 = t265 < t393;
    let t1097 = t1079 * t1096;
    let t1100 = 0.65854491829355115987e0_f64 * t989 * t386 - 0.65854491829355115987e0_f64 * t995 * t1000 + 0.65854491829355115987e0_f64 * t342 * t1073 - 0.65854491829355115987e0_f64 * t1076 * t1097;
    let t1102 = 1.0_f64 / t389;
    let t1106 = piecewise3(t394, t1100 * t1102 * t198 * t336 - t912 + t938 + t978 + t980 - t985, t895);
    (t1097, t1100, t1102, t1106)
}
