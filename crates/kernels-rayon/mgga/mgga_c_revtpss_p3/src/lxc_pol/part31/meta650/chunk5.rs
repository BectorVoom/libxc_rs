//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2149/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2149(t6317: f64, t7131: f64, t100025: f64, t100055: f64, t100160: f64, t100166: f64, t100230: f64, t1068: f64, t15670: f64, t1675: f64, t19745: f64, t19864: f64, t19986: f64, t20046: f64, t25580: f64, t27489: f64, t4831: f64, t4839: f64, t4907: f64, t7132: f64, t93752: f64) -> f64 {
    let t106971 = t6317 * t7131;
    let t106990 = 0.28582678745379824648e-3_f64 * t7132 * t20046 + 0.28582678745379824648e-3_f64 * t106971 * t1068 + 0.17149607247227894789e-2_f64 * t15670 * t7131 * t4839 - 0.57165357490759649296e-3_f64 * t100055 * t19986 - 0.85748036236139473944e-3_f64 * t100025 * t4907 - 0.42874018118069736972e-3_f64 * t25580 * t19745 - 0.57165357490759649296e-3_f64 * t93752 * t19864 - t100160 - 0.38110238327173099531e-3_f64 * t100166 + 0.57165357490759649296e-3_f64 * t100230 * t1675 + 0.57165357490759649296e-3_f64 * t27489 * t4831;
    t106990
}
