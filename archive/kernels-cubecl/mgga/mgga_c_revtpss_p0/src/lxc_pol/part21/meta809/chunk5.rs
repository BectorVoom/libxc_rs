//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2958/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2958<F: Float>(t53875: F, t225: F, t53014: F, t366: F, t11656: F, t15734: F, t1028: F, t11703: F, t11811: F, t11944: F, t15129: F, t15584: F, t15656: F, t15700: F, t15973: F, t16095: F, t16096: F, t16222: F, t3120: F, t3220: F, t42328: F, t4858: F, t4873: F, t53846: F, t53855: F, t53859: F, t53866: F) -> (F, F) {
    let t53876 = F::cast_from(0.14291339372689912324e-3_f64) * t53875;
    let t53877 = t53014 * t225;
    let t53878 = t53877 * t366;
    let t53881 = t11656 * t15734;
    let t53883 = F::cast_from(0.14291339372689912324e-2_f64) * t15700 * t16222 * t53846 + F::cast_from(0.42874018118069736972e-3_f64) * t42328 * t15584 * t4873 * t15973 - F::cast_from(0.12862205435420921092e-2_f64) * t53855 * t3120 - F::cast_from(0.42874018118069736972e-3_f64) * t53859 - F::cast_from(0.14291339372689912324e-2_f64) * t16095 * t11703 * t15129 * t16096 - F::cast_from(0.64311027177104605458e-3_f64) * t53866 * t1028 - F::cast_from(0.64311027177104605458e-3_f64) * t15656 * t3220 - F::cast_from(0.21437009059034868486e-3_f64) * t4858 * t11811 + t53876 - F::cast_from(0.12862205435420921092e-2_f64) * t53878 * t11944 + F::cast_from(0.30488190661738479624e-2_f64) * t53881;
    (t53877, t53883)
}
