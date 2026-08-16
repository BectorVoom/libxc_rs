//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1030/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1030(t11100: f64, t866: f64, t846: f64, t11002: f64, t11004: f64, t10982: f64, t10980: f64, t10986: f64, t11010: f64, t11015: f64, t11020: f64, t11024: f64, t11028: f64, t11033: f64, t11037: f64, t8605: f64, t8607: f64, t8616: f64, t8618: f64, t8756: f64) -> (f64, f64) {
    let t11101 = t11100 * t866;
    let t11103 = 1.0_f64 * t846 * t11101;
    let t11109 = 0.41203703703703703704e-2_f64 * t11002;
    let t11110 = 0.12361111111111111111e-1_f64 * t11004;
    let t11111 = 0.61805555555555555556e-2_f64 * t10982;
    let t11120 = -t8756 - 0.82407407407407407407e-2_f64 * t8616 + 0.20601851851851851852e-2_f64 * t8607 - 0.61805555555555555556e-2_f64 * t8618 + 0.30902777777777777778e-2_f64 * t8605 - 0.41203703703703703704e-2_f64 * t10980 + t11109 - t11110 + t11111 - 0.10300925925925925926e-1_f64 * t11010 + 0.37083333333333333333e-1_f64 * t11015 - 0.12361111111111111111e-1_f64 * t11020 - 0.61805555555555555555e-2_f64 * t11024 - 0.55625000000000000001e-1_f64 * t11028 + 0.37083333333333333334e-1_f64 * t11033 + 0.18541666666666666667e-1_f64 * t11037 - 0.92708333333333333333e-2_f64 * t10986;
    (t11103, t11120)
}
