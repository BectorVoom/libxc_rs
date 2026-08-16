//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2942/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2942(t16082: f64, t999: f64, t3155: f64, t3133: f64, t4900: f64, t11875: f64, t11927: f64, t15893: f64, t15907: f64, t15917: f64, t1592: f64, t15973: f64, t15975: f64, t16067: f64, t16076: f64, t19620: f64, t19639: f64, t3092: f64, t3117: f64, t357: f64, t42240: f64, t42249: f64, t42251: f64, t42550: f64, t42621: f64, t43050: f64, t4583: f64, t4899: f64) -> (f64, f64, f64) {
    let t53506 = t16082 * t999;
    let t53511 = t3155 * t999;
    let t53516 = t4900 * t3133;
    let t53528 = 0.64311027177104605458e-3_f64 * t11875 * t3117 * t16076 * t19639 - 0.42874018118069736972e-3_f64 * t15917 * t15975 - 0.42874018118069736972e-3_f64 * t4899 * t3092 * t4583 * t15973 + 0.14291339372689912324e-3_f64 * t16067 * t3092 * t1592 * t42550 * t357 - 0.38586616306262763275e-2_f64 * t42621 * t3117 * t15907 * t53506 + 0.25724410870841842183e-2_f64 * t43050 * t3117 * t15893 * t53511 + 0.64311027177104605458e-3_f64 * t16067 * t3117 * t15907 * t53516 + 0.12862205435420921092e-2_f64 * t11927 * t3117 * t16076 * t19620 + t42240 / 54.0_f64 - t42249 / 108.0_f64 - t42251 / 81.0_f64;
    (t53506, t53516, t53528)
}
