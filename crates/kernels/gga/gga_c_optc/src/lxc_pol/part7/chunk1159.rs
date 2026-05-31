//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1159/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1159<F: Float>(t2363: F, t7262: F, t2368: F, t7263: F, t1000: F, t1002: F, t1007: F, t1008: F, t10109: F, t1015: F, t10825: F, t10826: F, t23579: F, t2360: F, t23825: F, t24072: F, t24076: F, t24088: F, t24094: F, t2551: F, t4038: F, t7180: F, t7259: F, t914: F, t999: F) -> F {
    let t24096 = t7262 * t2363;
    let t24099 = t7263 * t2368;
    let t24105 = F::cast_from(16000000.0_f64) / F::cast_from(729.0_f64) * t24072 - t24076 - F::cast_from(4.0_f64) * t10109 * t7180 + t999 * t914 * t1000 * t23579 / F::cast_from(6.0_f64) + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t7263 * t2551 + F::cast_from(56.0_f64) / F::cast_from(27.0_f64) * t2360 * t7259 - F::cast_from(304700.0_f64) / F::cast_from(243.0_f64) * t1007 * t1008 * t24088 * t1015 + F::cast_from(20.0_f64) / F::cast_from(81.0_f64) * t24094 - F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t24096 * t1002 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t24099 + F::cast_from(28.0_f64) / F::cast_from(9.0_f64) * t4038 * t10825 * t10826 * t23825;
    t24105
}
