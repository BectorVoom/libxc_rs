//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1005/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1005<F: Float>(t50: F, t1897: F, t1900: F, t1940: F, t22034: F, t22035: F, t22041: F, t22046: F, t611: F, t6547: F, t6551: F, t6554: F, t22032: F, zeta_threshold: F) -> F {
    let t51 = t50 <= zeta_threshold;
    let t22050 = piecewise3::<f64>(t51, F::new(0.0), -F::new(56.0) / F::new(81.0) * t22034 * t22035 + F::new(16.0) / F::new(9.0) * t6547 * t1897 * t1900 - F::new(2.0) / F::new(3.0) * t1940 * t22041 - F::new(8.0) / F::new(9.0) * t6551 * t6554 + F::new(2.0) / F::new(3.0) * t611 * t22046);
    let t22052 = t22032 / F::new(2.0) + t22050 / F::new(2.0);
    t22052
}
