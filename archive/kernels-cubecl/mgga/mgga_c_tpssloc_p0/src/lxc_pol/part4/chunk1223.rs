//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 1223/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk1223<F: Float>(t19595: F, t20075: F, t20092: F, t20096: F, t19534: F, t510: F, t1458: F, t5107: F, t113: F, t12725: F, t1442: F, t1459: F, t1774: F, t1778: F, t1849: F, t19289: F, t19537: F, t2314: F, t4026: F, t4028: F, t4034: F, t4073: F, t4077: F, t5118: F, t513: F, t5361: F, t5460: F, t574: F, t652: F, t7458: F) -> F {
    let t20098 = t19595 + t20075 + t20092 + t20096;
    let t20100 = t510 * t19534;
    let t20109 = t5107 * t1458;
    let t20118 = -t113 * t19289 - F::cast_from(4.0_f64) * t12725 * t1459 - F::cast_from(2.0_f64) * t1442 * t5107 - F::cast_from(2.0_f64) * t1774 * t4026 + F::cast_from(2.0_f64) * t1778 * t5361 + F::cast_from(2.0_f64) * t1849 * t5118 + t19537 * t574 + t20098 * t513 - F::cast_from(2.0_f64) * t20100 * t652 - F::cast_from(4.0_f64) * t20109 * t652 - F::cast_from(4.0_f64) * t2314 * t5460 - F::cast_from(4.0_f64) * t4028 * t4073 - F::cast_from(4.0_f64) * t4028 * t4077 - F::cast_from(4.0_f64) * t4034 * t5460 - F::cast_from(4.0_f64) * t4073 * t7458;
    t20118
}
