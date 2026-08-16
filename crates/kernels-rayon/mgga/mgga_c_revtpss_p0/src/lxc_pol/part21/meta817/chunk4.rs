//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3009/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3009(t15830: f64, t3111: f64, t11866: f64, t16035: f64, t16088: f64, t342: f64, t380: f64, t11231: f64, t11703: f64, t11748: f64, t15153: f64, t15719: f64, t15837: f64, t16089: f64, t19705: f64, t247: f64, t3092: f64, t3116: f64, t4834: f64, t53835: f64, t54982: f64, t54983: f64, t54988: f64, t54991: f64, t54994: f64, t55000: f64, t906: f64) -> (f64, f64) {
    let t55002 = t15830 * t3111;
    let t55004 = t11866 * t16035;
    let t55011 = t342 * t380 * t16088;
    let t55016 = 0.85748036236139473944e-3_f64 * t4834 * t11748 + 0.51448821741683684368e-2_f64 * t54982 * t247 * t3116 * t54983 + 0.20579528696673473747e-1_f64 * t54988 * t15719 - 0.85748036236139473944e-3_f64 * t54991 - 0.85748036236139473944e-3_f64 * t54994 + 0.85748036236139473944e-3_f64 * t16089 * t3092 * t19705 * t53835 - 0.57165357490759649295e-3_f64 * t55000 - 0.30488190661738479624e-2_f64 * t55002 - 0.85748036236139473944e-3_f64 * t55004 + 0.85748036236139473944e-3_f64 * t16089 * t3092 * t15837 * t906 - 0.42874018118069736972e-2_f64 * t55011 * t11703 * t15153 * t11231;
    (t55011, t55016)
}
