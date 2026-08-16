//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1373/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1373(t11013: f64, t225: f64, t10163: f64, t386: f64, t68: f64, t3175: f64, t11008: f64, t10160: f64, t10165: f64, t10167: f64, t10170: f64, t1052: f64, t1055: f64, t1058: f64, t1060: f64, t1061: f64, t1065: f64, t1066: f64, t11010: f64, t11024: f64, t11027: f64, t11028: f64, t11034: f64, t11046: f64, t11048: f64, t11051: f64, t11054: f64, t11061: f64, t11067: f64, t11077: f64, t11078: f64, t11084: f64, t11085: f64, t14630: f64, t3026: f64, t3076: f64, t3120: f64, t3166: f64, t3169: f64, t3174: f64, t3176: f64, t3180: f64, t3186: f64, t3188: f64, t3192: f64, t3193: f64, t3196: f64, t3197: f64, t3200: f64, t3202: f64, t3204: f64, t3206: f64, t3207: f64, t381: f64, t384: f64, t388: f64, t42715: f64, t43082: f64, t43083: f64, t43470: f64, t43473: f64, t43483: f64, t43504: f64, t43512: f64, t43515: f64, t43516: f64, t43525: f64, t43536: f64, t43542: f64, t43584: f64, t4684: f64) -> f64 {
    let t43599 = t11013 * t225;
    let t43603 = 1.0_f64 / t10163 / t386;
    let t43604 = t68 * t43603;
    let t43605 = t3175 * t3175;
    let t43619 = t11008 * t225;
    let t43622 = -6.0_f64 * t10170 * t3207 - t1052 * t1055 * (6.0_f64 * t1058 * t3166 * t3120 * t1060 - 4.0_f64 * t3200 * t11027 * t4684 + 24.0_f64 * t3186 * t3192 * t11054 - 12.0_f64 * t3200 * t11077 * t4684 + 12.0_f64 * t11051 * t3193 + 24.0_f64 * t43473 * t11061 - 24.0_f64 * t43470 * t11067 + 12.0_f64 * t3180 * t11078 + 6.0_f64 * t3076 * t3204 + t43083 * t384 + t43512 + t1058 * t381 * t42715 * t1060 + 4.0_f64 * t11046 * t43483 * t11048 + 6.0_f64 * t11046 * t3196 * t14630 + 6.0_f64 * t3186 * t43525 * t3188 + 14.0_f64 * t43515 * t43504 * t43516 + 4.0_f64 * t43542 * t1061 + 24.0_f64 * t11034 * t11024 + 4.0_f64 * t3180 * t11028 + 6.0_f64 * t11051 * t3197 - 6.0_f64 * t43536 * t3202 + t43584) + 12.0_f64 * t11010 * t3176 - 24.0_f64 * t3026 * t10167 - 24.0_f64 * t3169 * t10167 - 12.0_f64 * t10160 * t3207 + t43082 * t381 * t388 - 12.0_f64 * t43599 * t1066 + 24.0_f64 * t1052 * t43604 * t43605 - 4.0_f64 * t3026 * t11085 - 36.0_f64 * t1052 * t10165 * t3175 * t3206 + 8.0_f64 * t1052 * t3174 * t1065 * t11084 - 4.0_f64 * t43619 * t1066;
    t43622
}
