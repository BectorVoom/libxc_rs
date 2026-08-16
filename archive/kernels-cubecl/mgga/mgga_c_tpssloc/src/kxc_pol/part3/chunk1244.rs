//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1244/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1244<F: Float>(t12215: F, t12335: F, t12340: F, t12346: F, t12356: F, t12358: F, t12366: F, t12386: F, t12388: F, t12395: F, t12429: F, t16366: F, t16370: F, t16379: F, t16383: F, t16387: F, t16391: F, t16394: F, t16400: F, t16401: F, t16405: F, t3803: F, t3809: F, t5246: F, t5252: F, t5303: F) -> F {
    let t16411 = -t12335 + t12429 * t5303 / F::cast_from(384.0_f64) + t3803 * t16366 / F::cast_from(384.0_f64) + t3803 * t16370 / F::cast_from(768.0_f64) + F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t12340 - F::cast_from(119.0_f64) / F::cast_from(1728.0_f64) * t12346 - F::cast_from(35.0_f64) / F::cast_from(1152.0_f64) * t12356 + F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t12358 - F::cast_from(119.0_f64) / F::cast_from(6912.0_f64) * t12366 - t12215 * t16379 / F::cast_from(4.0_f64) + t3803 * t16383 / F::cast_from(768.0_f64) + t5246 * t16387 / F::cast_from(512.0_f64) - t5246 * t16391 / F::cast_from(384.0_f64) + t16394 * t3809 / F::cast_from(384.0_f64) - t16400 + t16401 * t5252 / F::cast_from(768.0_f64) - F::cast_from(5.0_f64) / F::cast_from(768.0_f64) * t3803 * t16405 - F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t12386 + F::cast_from(7.0_f64) / F::cast_from(4608.0_f64) * t12388 + F::cast_from(7.0_f64) / F::cast_from(4608.0_f64) * t12395;
    t16411
}
