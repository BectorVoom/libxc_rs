//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1067/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1067(t14656: f64, t285: f64, t4923: f64, t8772: f64, t3908: f64, t912: f64, t2593: f64, t4939: f64, t905: f64, t10980: f64, t11109: f64, t11110: f64, t11111: f64, t14459: f64, t14492: f64, t14495: f64, t14505: f64, t14507: f64, t14517: f64, t14521: f64, t14525: f64, t14528: f64, t14532: f64, t14535: f64, t8616: f64, t8756: f64) -> (f64, f64, f64, f64) {
    let t14658 = 0.621814e-1_f64 * t14656 * t285;
    let t14659 = t8772 * t4923;
    let t14660 = t14659 * t3908;
    let t14662 = 0.10389515463408878255e3_f64 * t912 * t14660;
    let t14663 = t2593 * t4939;
    let t14664 = t14663 * t905;
    let t14666 = 0.11696447245269292414e1_f64 * t912 * t14664;
    let t14680 = -t8756 - 0.41203703703703703703e-2_f64 * t8616 - 0.82407407407407407408e-2_f64 * t10980 + t11109 - t11110 + t11111 + 0.20601851851851851852e-2_f64 * t14495 - 0.10300925925925925926e-1_f64 * t14517 + 0.37083333333333333333e-1_f64 * t14459 - 0.12361111111111111111e-1_f64 * t14521 - 0.61805555555555555557e-2_f64 * t14505 - 0.55625000000000000001e-1_f64 * t14525 + 0.37083333333333333334e-1_f64 * t14528 + 0.30902777777777777778e-2_f64 * t14507 - 0.61805555555555555555e-2_f64 * t14532 + 0.18541666666666666667e-1_f64 * t14535 - 0.92708333333333333333e-2_f64 * t14492;
    (t14658, t14662, t14666, t14680)
}
