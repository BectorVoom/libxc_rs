//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1026/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1026<F: Float>(t50: F, t1026: F, t99: F, t1896: F, t1897: F, t1900: F, t22035: F, t22041: F, t22046: F, t52: F, t6554: F, t6724: F, t6727: F, zeta_threshold: F) -> F {
    let t51 = t50 <= zeta_threshold;
    let t22323 = F::cast_from(1.0_f64) / t99 / t1026;
    let t22336 = piecewise3::<F>(t51, F::cast_from(0.0_f64), F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t22323 * t22035 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t6724 * t1897 * t1900 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1896 * t22041 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t6727 * t6554 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t52 * t22046);
    t22336
}
