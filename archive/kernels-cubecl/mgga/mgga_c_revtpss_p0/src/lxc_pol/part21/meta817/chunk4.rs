//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3009/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3009<F: Float>(t15830: F, t3111: F, t11866: F, t16035: F, t16088: F, t342: F, t380: F, t11231: F, t11703: F, t11748: F, t15153: F, t15719: F, t15837: F, t16089: F, t19705: F, t247: F, t3092: F, t3116: F, t4834: F, t53835: F, t54982: F, t54983: F, t54988: F, t54991: F, t54994: F, t55000: F, t906: F) -> (F, F) {
    let t55002 = t15830 * t3111;
    let t55004 = t11866 * t16035;
    let t55011 = t342 * t380 * t16088;
    let t55016 = F::cast_from(0.85748036236139473944e-3_f64) * t4834 * t11748 + F::cast_from(0.51448821741683684368e-2_f64) * t54982 * t247 * t3116 * t54983 + F::cast_from(0.20579528696673473747e-1_f64) * t54988 * t15719 - F::cast_from(0.85748036236139473944e-3_f64) * t54991 - F::cast_from(0.85748036236139473944e-3_f64) * t54994 + F::cast_from(0.85748036236139473944e-3_f64) * t16089 * t3092 * t19705 * t53835 - F::cast_from(0.57165357490759649295e-3_f64) * t55000 - F::cast_from(0.30488190661738479624e-2_f64) * t55002 - F::cast_from(0.85748036236139473944e-3_f64) * t55004 + F::cast_from(0.85748036236139473944e-3_f64) * t16089 * t3092 * t15837 * t906 - F::cast_from(0.42874018118069736972e-2_f64) * t55011 * t11703 * t15153 * t11231;
    (t55011, t55016)
}
