//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 879/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk879(t51: f64, t101: f64, t1217: f64, t3008: f64, t3011: f64, t419: f64, t8584: f64, t906: f64, t9353: f64, t552: f64, t551: f64, t5136: f64, t549: f64, t6310: f64, t6324: f64, t6333: f64, t8056: f64, t8062: f64, t8065: f64, t8069: f64, t8073: f64, t8076: f64, t8080: f64, t8084: f64, t8086: f64, t8092: f64, t9335: f64, t9339: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t52 = t51 <= zeta_threshold;
    let t9363 = piecewise3(t52, 0.0_f64, -10.0_f64 / 27.0_f64 * t3008 * t419 - 40.0_f64 / 9.0_f64 * t906 * t1217 + 10.0_f64 / 9.0_f64 * t3011 * t419 + 5.0_f64 / 3.0_f64 * t101 * t8584);
    let t9365 = t9353 / 2.0_f64 + t9363 / 2.0_f64;
    let t9366 = t552 * t9365;
    let t9367 = t551 * t9366;
    let t9370 = 0.69861909304693186869e-1_f64 * t9335 - t8056 - 0.2600466522016280569e0_f64 * t5136 * t9339 + 0.58544643236296698111e-1_f64 * t8062 - t8065 - t8069 - t8073 + t8076 - t8080 + t8084 + 0.34930954652346593433e-1_f64 * t8086 + t8092 - 0.43341108700271342816e-1_f64 * t549 * t9367 - t6310 + t6324 + t6333;
    (t9365, t9366, t9370)
}
