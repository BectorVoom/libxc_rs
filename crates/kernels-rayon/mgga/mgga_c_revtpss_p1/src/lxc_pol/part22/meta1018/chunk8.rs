//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3529/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3529(t1045: f64, t11774: f64, t11852: f64, t15140: f64, t15614: f64, t15692: f64, t15696: f64, t15700: f64, t16229: f64, t1668: f64, t19986: f64, t372: f64, t42328: f64, t42907: f64, t43082: f64, t53914: f64, t54667: f64, t54678: f64, t54680: f64, t54687: f64, t54693: f64, t54704: f64, t54708: f64, t66689: f64) -> f64 {
    let t66893 = 0.1270341277572436651e-2_f64 * t15700 * t372 * t11852 * t1668 * t1045 * t15140 - 0.57165357490759649296e-3_f64 * t53914 * t19986 - 0.57165357490759649296e-3_f64 * t11774 * t15696 * t15614 + 0.6351706387862183255e-3_f64 * t54667 + 0.30488190661738479624e-2_f64 * t54678 - 0.57165357490759649296e-3_f64 * t54680 - 0.1270341277572436651e-3_f64 * t54687 - 0.28582678745379824648e-3_f64 * t54693 - 0.57165357490759649296e-3_f64 * t54704 + 0.19055119163586549765e-3_f64 * t54708 - 0.6351706387862183255e-4_f64 * t42907 - 0.11433071498151929859e-2_f64 * t43082 * t66689 * t16229 + 0.57165357490759649296e-3_f64 * t42328 * t66689 * t15692;
    t66893
}
