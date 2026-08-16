//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 1220/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk1220(t20034: f64, t20062: f64, t1390: f64, t6463: f64, t12044: f64, t12048: f64, t12057: f64, t12059: f64, t1297: f64, t1307: f64, t1388: f64, t15898: f64, t15911: f64, t15916: f64, t15917: f64, t15923: f64, t193: f64, t19596: f64, t19599: f64, t19603: f64, t19631: f64, t19677: f64, t3918: f64, t5126: f64, t5160: f64, t5161: f64, t533: f64, t5356: f64, t571: f64, t6330: f64, t9780: f64, t9789: f64) -> f64 {
    let t20063 = t20034 + t20062;
    let t20067 = t6463 * t1390;
    let t20075 = -t15898 + t9780 - t5160 * t19596 * t1388 + t19599 + t12044 + t15911 - t12048 - 2.0_f64 * t5160 * t5161 * t5356 + 12.0_f64 * t5126 * t19603 + 3.0_f64 * t193 * t1297 * t19631 + t193 * t533 * t20063 * t1390 + 3.0_f64 * t3918 * t20067 * t1307 + t19677 - t15916 - t15917 - t12057 + 6.0_f64 * t193 * t1307 * t571 * t6330 - t12059 + t15923 - t9789;
    t20075
}
