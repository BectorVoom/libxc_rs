//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 728/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk728<F: Float>(t1213: F, t1227: F, t1737: F, t1748: F, t3506: F, t3515: F, t3542: F, t3547: F, t467: F, t5005: F, t5019: F, t5024: F, t5036: F, t5041: F, t6109: F, t6203: F, t6207: F, t6211: F, t6221: F, t6227: F, t6232: F) -> F {
    let t6237 = -t5005 * t1748 / F::cast_from(2304.0_f64) - t5019 * t1737 / F::cast_from(288.0_f64) + F::cast_from(5.0_f64) / F::cast_from(13824.0_f64) * t1227 * t6203 - t1227 * t6207 / F::cast_from(4608.0_f64) - t1227 * t6211 / F::cast_from(2304.0_f64) - t5036 / F::cast_from(54.0_f64) + F::cast_from(11.0_f64) / F::cast_from(108.0_f64) * t6109 * t467 - t5041 / F::cast_from(432.0_f64) - t3542 + t1213 * t6221 / F::cast_from(3072.0_f64) + t3506 * t6227 / F::cast_from(1536.0_f64) - t3515 * t6232 / F::cast_from(3072.0_f64) + t5024 * t1748 / F::cast_from(432.0_f64) - t3547;
    t6237
}
