//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2088/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2088(t10383: f64, t1926: f64, t3014: f64, t40: f64, t1933: f64, t23479: f64, t1004: f64, t23528: f64, t23544: f64, t3053: f64, t10948: f64, t23536: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t83028 = 5.0_f64 / 1296.0_f64 * t1926 * t10383;
    let t83032 = t40 * t3014;
    let t83034 = t1933 * t83032 * t23479;
    let t83038 = t1004 * t23528;
    let t83041 = t23544 * t3053;
    let t83043 = t10948 * t23536;
    (t83028, t83032, t83034, t83038, t83041, t83043)
}
