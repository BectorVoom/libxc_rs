//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1117/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1117(t7780: f64, t7784: f64, t20: f64, t251: f64, t2865: f64, t1240: f64, t27055: f64, t7788: f64, t3611: f64, t7794: f64, t5329: f64, t2197: f64, t26841: f64, t26844: f64, t26846: f64, t26849: f64, t26852: f64, t26966: f64, t26977: f64, t27014: f64, t27070: f64, t27077: f64, t7775: f64, t7796: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t27080 = t7780 * t7784;
    let t27083 = t251 * t2865 * t20;
    let t27084 = t1240 * t27083;
    let t27087 = t7788 * t27055;
    let t27089 = t7794 * t3611;
    let t27090 = t5329 * t27089;
    let t27093 = -0.61905925925925925925e-2_f64 * t26841 + 0.11607361111111111111e-2_f64 * t26844 - 0.23214722222222222222e-2_f64 * t26846 + 0.23214722222222222222e-2_f64 * t26849 - 0.18534722222222222222e-2_f64 * t26966 * t7796 - 0.18534722222222222222e-2_f64 * t26966 * t7775 + 0.69505208333333333334e-3_f64 * t27014 * t7775 + 0.92754700520833333334e-4_f64 * t27070 * t7775 - 0.92858888888888888886e-2_f64 * t26852 + 0.69505208333333333334e-3_f64 * t27014 * t7796 - 0.92835860883789062501e-5_f64 * t27077 * t26977 + 0.61782407407407407408e-3_f64 * t27080 - 0.33980324074074074074e-2_f64 * t27084 * t2197 + 0.23168402777777777778e-3_f64 * t27087 + 0.34752604166666666667e-3_f64 * t7788 * t27090;
    (t27080, t27083, t27084, t27087, t27089, t27090, t27093)
}
