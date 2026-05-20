//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3763/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3763<F: Float>(t1261: F, t1264: F, t12809: F, t12866: F, t12910: F, t16696: F, t17369: F, t17412: F, t17459: F, t17649: F, t17661: F, t17668: F, t17736: F, t20956: F, t21028: F, t21035: F, t21164: F, t21182: F, t21257: F, t247: F, t3626: F, t3720: F, t44980: F, t45371: F, t5381: F, t5397: F, t5405: F, t5406: F, t57548: F, t59062: F, t59269: F, t59330: F, t60927: F, t68251: F, t71452: F) -> F {
    let t71824 = F::cast_from(0.85748036236139473944e-3_f64) * t12809 * t3720 * t21164 * t21028 + F::cast_from(0.17149607247227894789e-2_f64) * t12910 * t3720 * t21257 * t17459 - F::cast_from(0.22866142996303859718e-2_f64) * t17736 * t3626 * t21035 * t71452 + F::cast_from(0.28582678745379824648e-3_f64) * t12866 * t17649 * t21182 * t5405 - t44980 / F::new(972.0) + F::cast_from(0.57165357490759649296e-3_f64) * t12866 * t59062 * t5406 + F::cast_from(0.57165357490759649296e-3_f64) * t12866 * t17661 * t17668 - F::cast_from(0.42874018118069736972e-3_f64) * t45371 * t3720 * t20956 * t16696 - F::cast_from(0.31758531939310916275e-3_f64) * t59269 - t57548 * t59330 * t60927 / F::new(12.0) + F::cast_from(0.30488190661738479624e-2_f64) * t17412 * t5397 - F::cast_from(0.57165357490759649296e-3_f64) * t1261 * t247 * t1264 * t68251 - F::cast_from(0.28582678745379824648e-3_f64) * t5381 * t17369;
    t71824
}
