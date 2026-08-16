//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2203/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2203<F: Float>(t1408: F, t4119: F, t193: F, t7637: F, t1530: F, t4303: F, t25373: F, t22960: F, t67123: F, t1877: F, t1915: F, t22959: F, t23290: F, t25028: F, t2522: F, t25358: F, t25372: F, t25375: F, t25381: F, t28448: F, t28462: F, t6542: F, t6670: F, t7541: F, t7545: F, t86836: F, t97990: F, t98000: F, t98004: F, t98008: F, t98012: F, t98015: F) -> (F, F, F) {
    let t98020 = t1408 * t4119;
    let t98027 = t193 * t7637;
    let t98030 = t1530 * t4303;
    let t98031 = t25373 * t98030;
    let t98034 = t22960 * t67123;
    let t98039 = -t1877 * t6670 * t97990 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t28448 * t6542 + F::cast_from(3.0_f64) * t2522 * t7541 * t25028 - F::cast_from(3.0_f64) * t25372 * t98000 + F::cast_from(3.0_f64) * t22959 * t98004 - F::cast_from(3.0_f64) * t22959 * t98008 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t22959 * t98012 - F::cast_from(3.0_f64) * t22959 * t98015 - t1877 * t25358 * t25381 + F::cast_from(3.0_f64) * t2522 * t1915 * t98020 - t1877 * t23290 * t28462 / F::cast_from(2.0_f64) + F::cast_from(2.0_f64) * t98027 * t25375 + F::cast_from(2.0_f64) * t25372 * t98031 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t22959 * t98034 - t1877 * t86836 * t7545;
    (t98027, t98030, t98039)
}
