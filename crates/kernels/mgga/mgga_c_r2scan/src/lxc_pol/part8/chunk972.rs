//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 972/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk972<F: Float>(t552: F, t9365: F, t551: F, t5136: F, t549: F, t6310: F, t6324: F, t6333: F, t8056: F, t8062: F, t8065: F, t8069: F, t8073: F, t8076: F, t8080: F, t8084: F, t8086: F, t8092: F, t9335: F, t9339: F) -> (F, F) {
    let t9366 = t552 * t9365;
    let t9367 = t551 * t9366;
    let t9370 = 0.69861909304693186869e-1 * t9335 - t8056 - 0.2600466522016280569e0 * t5136 * t9339 + 0.58544643236296698111e-1 * t8062 - t8065 - t8069 - t8073 + t8076 - t8080 + t8084 + 0.34930954652346593433e-1 * t8086 + t8092 - 0.43341108700271342816e-1 * t549 * t9367 - t6310 + t6324 + t6333;
    (t9367, t9370)
}
