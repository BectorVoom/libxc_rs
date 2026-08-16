//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3542/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3542(t15745: f64, t4845: f64, t1011: f64, t1012: f64, t1045: f64, t11774: f64, t11927: f64, t15149: f64, t15651: f64, t15656: f64, t15691: f64, t15696: f64, t15700: f64, t15958: f64, t1665: f64, t19620: f64, t20089: f64, t3117: f64, t3236: f64, t4782: f64, t4854: f64, t4858: f64, t53866: f64, t54384: f64, t54818: f64, t55104: f64, t55148: f64, t55150: f64, t60717: f64) -> f64 {
    let t67301 = t15745 * t4845;
    let t67318 = 0.17149607247227894789e-2_f64 * t11927 * t3117 * t20089 * t19620 - 0.57165357490759649296e-3_f64 * t11774 * t54818 * t4782 - 0.3811023832717309953e-3_f64 * t55104 - t1011 * t1012 * t3236 * t60717 / 72.0_f64 - 0.14481890564325777821e-1_f64 * t54384 * t1665 + 0.30488190661738479624e-2_f64 * t67301 - 0.42874018118069736972e-3_f64 * t53866 * t1665 - 0.85748036236139473944e-3_f64 * t15656 * t4854 - 0.42874018118069736972e-3_f64 * t4858 * t15651 - 0.57165357490759649296e-3_f64 * t11774 * t15696 * t15958 - 0.57165357490759649296e-3_f64 * t15700 * t15691 * t1045 * t15149 - 0.28582678745379824648e-3_f64 * t55148 + 0.30488190661738479624e-2_f64 * t55150;
    t67318
}
