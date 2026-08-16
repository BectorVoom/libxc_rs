//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1151/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1151<F: Float>(t191: F, t529: F, t864: F, t2437: F, t2433: F, t10127: F, t10615: F, t2364: F, t23821: F, t23823: F, t23825: F, t23946: F, t23951: F, t2544: F, t2551: F, t2722: F, t3608: F, t4038: F, t4044: F, t7180: F, t7263: F, t7301: F, t7304: F) -> F {
    let t23957 = t529 * t864 * t191;
    let t23958 = t23957 * t2437;
    let t23959 = t2433 * t23958;
    let t23963 = -t23821 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t23823 - F::cast_from(8.0_f64) * t4038 * t3608 * t10615 * t23825 + t7263 * t2544 + F::cast_from(128.0_f64) / F::cast_from(9.0_f64) * t2364 * t7301 - F::cast_from(64.0_f64) / F::cast_from(9.0_f64) * t7304 * t2551 - t23946 + F::cast_from(6.0_f64) * t4038 * t2722 * t4044 * t23825 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t4038 * t3608 * t4044 * t23951 - F::cast_from(400.0_f64) / F::cast_from(243.0_f64) * t23959 + F::cast_from(32.0_f64) / F::cast_from(3.0_f64) * t10127 * t7180;
    t23963
}
