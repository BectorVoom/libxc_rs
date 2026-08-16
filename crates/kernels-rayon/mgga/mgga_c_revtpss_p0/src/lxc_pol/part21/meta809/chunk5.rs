//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2958/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2958(t53875: f64, t225: f64, t53014: f64, t366: f64, t11656: f64, t15734: f64, t1028: f64, t11703: f64, t11811: f64, t11944: f64, t15129: f64, t15584: f64, t15656: f64, t15700: f64, t15973: f64, t16095: f64, t16096: f64, t16222: f64, t3120: f64, t3220: f64, t42328: f64, t4858: f64, t4873: f64, t53846: f64, t53855: f64, t53859: f64, t53866: f64) -> (f64, f64) {
    let t53876 = 0.14291339372689912324e-3_f64 * t53875;
    let t53877 = t53014 * t225;
    let t53878 = t53877 * t366;
    let t53881 = t11656 * t15734;
    let t53883 = 0.14291339372689912324e-2_f64 * t15700 * t16222 * t53846 + 0.42874018118069736972e-3_f64 * t42328 * t15584 * t4873 * t15973 - 0.12862205435420921092e-2_f64 * t53855 * t3120 - 0.42874018118069736972e-3_f64 * t53859 - 0.14291339372689912324e-2_f64 * t16095 * t11703 * t15129 * t16096 - 0.64311027177104605458e-3_f64 * t53866 * t1028 - 0.64311027177104605458e-3_f64 * t15656 * t3220 - 0.21437009059034868486e-3_f64 * t4858 * t11811 + t53876 - 0.12862205435420921092e-2_f64 * t53878 * t11944 + 0.30488190661738479624e-2_f64 * t53881;
    (t53877, t53883)
}
