//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1095/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1095(t2857: f64, t4105: f64, t11873: f64, t11875: f64, t11942: f64, t11880: f64, t11885: f64, t11890: f64, t11896: f64, t11899: f64, t11904: f64, t11908: f64, t11938: f64, t11952: f64, t9221: f64, t9223: f64, t9226: f64, t9228: f64, t9331: f64) -> (f64, f64) {
    let t11982 = 2.0_f64 * t2857 * t4105;
    let t11988 = 0.41203703703703703704e-2_f64 * t11873;
    let t11989 = 0.12361111111111111111e-1_f64 * t11875;
    let t11990 = 0.61805555555555555556e-2_f64 * t11942;
    let t11999 = -t9331 + 0.82407407407407407407e-2_f64 * t9221 + 0.20601851851851851852e-2_f64 * t9223 - 0.61805555555555555556e-2_f64 * t9226 - 0.30902777777777777778e-2_f64 * t9228 + 0.41203703703703703704e-2_f64 * t11938 + t11988 - t11989 - t11990 + 0.10300925925925925926e-1_f64 * t11880 - 0.37083333333333333333e-1_f64 * t11885 - 0.12361111111111111111e-1_f64 * t11890 - 0.61805555555555555555e-2_f64 * t11896 + 0.55625000000000000001e-1_f64 * t11899 + 0.37083333333333333334e-1_f64 * t11904 + 0.18541666666666666667e-1_f64 * t11908 + 0.92708333333333333333e-2_f64 * t11952;
    (t11982, t11999)
}
