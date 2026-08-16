//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1019/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1019(t33947: f64, t814: f64, t1509: f64, t8728: f64, t114659: f64, t114666: f64, t114670: f64, t116673: f64, t116681: f64, t121493: f64, t121498: f64, t121501: f64, t121504: f64, t121509: f64, t121517: f64, t121521: f64, t121524: f64, t121528: f64, t121533: f64, t121536: f64, t1510: f64, t31994: f64, t4166: f64, t4291: f64, t812: f64, t829: f64) -> (f64, f64) {
    let t123622 = t814 * t33947;
    let t123626 = t8728 * t1509;
    let t123641 = 0.6579736267392905746e-1_f64 * t121493 + 0.6579736267392905746e-1_f64 * t121498 + 0.3289868133696452873e-1_f64 * t121501 - 0.16449340668482264365e-1_f64 * t121504 - t812 * t123622 * t829 - 0.6579736267392905746e-1_f64 * t121509 - t4291 * t123626 * t829 + 0.15352717957250113407e0_f64 * t114659 + 0.3289868133696452873e-1_f64 * t114666 - 0.6579736267392905746e-1_f64 * t121517 - 0.6579736267392905746e-1_f64 * t121521 + 0.16449340668482264365e-1_f64 * t121524 - 0.3289868133696452873e-1_f64 * t121528 - t812 * t116681 * t1510 + 0.15352717957250113407e0_f64 * t121533 - 0.76763589786250567037e-1_f64 * t114670 + t116673 + 0.76763589786250567037e-1_f64 * t121536 - t4166 * t31994;
    (t123626, t123641)
}
