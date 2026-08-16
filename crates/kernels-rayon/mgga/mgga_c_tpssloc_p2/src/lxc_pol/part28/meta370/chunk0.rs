//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1403/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1403(t14328: f64, t932: f64, t4446: f64, t942: f64, t1573: f64, t2929: f64, t13716: f64, t951: f64, t13563: f64, t13566: f64, t13602: f64, t10556: f64, t10558: f64, t10560: f64, t10562: f64, t10608: f64, t13569: f64, t13572: f64, t13575: f64, t13578: f64, t13581: f64, t13584: f64, t13587: f64, t13598: f64, t13613: f64) -> (f64, f64, f64, f64, f64) {
    let t14329 = t14328 * t932;
    let t14332 = t4446 * t942;
    let t14337 = t1573 * t2929;
    let t14344 = t13716 * t951;
    let t14352 = 0.41203703703703703704e-2_f64 * t13563;
    let t14353 = 0.12361111111111111111e-1_f64 * t13566;
    let t14354 = 0.61805555555555555556e-2_f64 * t13602;
    let t14363 = -t10608 - 0.82407407407407407407e-2_f64 * t10556 + 0.20601851851851851852e-2_f64 * t10558 - 0.61805555555555555556e-2_f64 * t10560 + 0.30902777777777777778e-2_f64 * t10562 - 0.41203703703703703704e-2_f64 * t13598 + t14352 - t14353 + t14354 - 0.10300925925925925926e-1_f64 * t13569 + 0.37083333333333333333e-1_f64 * t13572 - 0.12361111111111111111e-1_f64 * t13575 - 0.61805555555555555555e-2_f64 * t13578 - 0.55625000000000000001e-1_f64 * t13581 + 0.37083333333333333334e-1_f64 * t13584 + 0.18541666666666666667e-1_f64 * t13587 - 0.92708333333333333333e-2_f64 * t13613;
    (t14329, t14332, t14337, t14344, t14363)
}
