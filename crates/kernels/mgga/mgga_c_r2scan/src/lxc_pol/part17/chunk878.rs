//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 878/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk878<F: Float>(t51: F, t101: F, t1217: F, t3008: F, t3011: F, t419: F, t8584: F, t906: F, t9353: F, t552: F, t551: F, t5136: F, t549: F, t6310: F, t6324: F, t6333: F, t8056: F, t8062: F, t8065: F, t8069: F, t8073: F, t8076: F, t8080: F, t8084: F, t8086: F, t8092: F, t9335: F, t9339: F, zeta_threshold: F) -> (F, F, F) {
    let t52 = t51 <= zeta_threshold;
    let t9363 = piecewise3::<f64>(t52, F::new(0.0), -F::new(10.0) / F::new(27.0) * t3008 * t419 - F::new(40.0) / F::new(9.0) * t906 * t1217 + F::new(10.0) / F::new(9.0) * t3011 * t419 + F::new(5.0) / F::new(3.0) * t101 * t8584);
    let t9365 = t9353 / F::new(2.0) + t9363 / F::new(2.0);
    let t9366 = t552 * t9365;
    let t9367 = t551 * t9366;
    let t9370 = F::new(0.69861909304693186869e-1) * t9335 - t8056 - F::new(0.2600466522016280569e0) * t5136 * t9339 + F::new(0.58544643236296698111e-1) * t8062 - t8065 - t8069 - t8073 + t8076 - t8080 + t8084 + F::new(0.34930954652346593433e-1) * t8086 + t8092 - F::new(0.43341108700271342816e-1) * t549 * t9367 - t6310 + t6324 + t6333;
    (t9365, t9366, t9370)
}
