//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3483/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3483(t16158: f64, t4834: f64, t19791: f64, t3127: f64, t3172: f64, t1042: f64, t11977: f64, t11994: f64, t15839: f64, t15847: f64, t15850: f64, t16149: f64, t1675: f64, t19649: f64, t19878: f64, t19930: f64, t19940: f64, t2858: f64, t3188: f64, t4831: f64, t4837: f64, t4875: f64, t53353: f64, t53926: f64, t54137: f64, t6302: f64) -> f64 {
    let t65538 = t4834 * t16158;
    let t65553 = t3127 * t3172 * t19791;
    let t65563 = -0.57165357490759649296e-3_f64 * t11994 * t19940 + 0.57165357490759649296e-3_f64 * t53353 + 0.3811023832717309953e-3_f64 * t65538 + 0.85748036236139473944e-3_f64 * t19878 * t15839 + 0.28582678745379824648e-3_f64 * t54137 * t1675 + 0.57165357490759649296e-3_f64 * t15850 * t4831 + 0.28582678745379824648e-3_f64 * t4834 * t15847 - 0.57165357490759649296e-3_f64 * t4837 * t1042 * t19649 * t2858 - 0.3811023832717309953e-3_f64 * t65553 + 0.57165357490759649296e-3_f64 * t19878 * t16149 + 0.30488190661738479624e-2_f64 * t53926 * t4875 + 0.17149607247227894789e-2_f64 * t3188 * t19930 - 0.22866142996303859718e-2_f64 * t11977 * t6302;
    t65563
}
