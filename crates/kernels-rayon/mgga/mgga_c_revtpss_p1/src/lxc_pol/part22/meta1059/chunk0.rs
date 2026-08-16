//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3763/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3763(t1261: f64, t1264: f64, t12809: f64, t12866: f64, t12910: f64, t16696: f64, t17369: f64, t17412: f64, t17459: f64, t17649: f64, t17661: f64, t17668: f64, t17736: f64, t20956: f64, t21028: f64, t21035: f64, t21164: f64, t21182: f64, t21257: f64, t247: f64, t3626: f64, t3720: f64, t44980: f64, t45371: f64, t5381: f64, t5397: f64, t5405: f64, t5406: f64, t57548: f64, t59062: f64, t59269: f64, t59330: f64, t60927: f64, t68251: f64, t71452: f64) -> f64 {
    let t71824 = 0.85748036236139473944e-3_f64 * t12809 * t3720 * t21164 * t21028 + 0.17149607247227894789e-2_f64 * t12910 * t3720 * t21257 * t17459 - 0.22866142996303859718e-2_f64 * t17736 * t3626 * t21035 * t71452 + 0.28582678745379824648e-3_f64 * t12866 * t17649 * t21182 * t5405 - t44980 / 972.0_f64 + 0.57165357490759649296e-3_f64 * t12866 * t59062 * t5406 + 0.57165357490759649296e-3_f64 * t12866 * t17661 * t17668 - 0.42874018118069736972e-3_f64 * t45371 * t3720 * t20956 * t16696 - 0.31758531939310916275e-3_f64 * t59269 - t57548 * t59330 * t60927 / 12.0_f64 + 0.30488190661738479624e-2_f64 * t17412 * t5397 - 0.57165357490759649296e-3_f64 * t1261 * t247 * t1264 * t68251 - 0.28582678745379824648e-3_f64 * t5381 * t17369;
    t71824
}
