//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 843/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk843<F: Float>(t2650: F, t2701: F, t2704: F, t2797: F, t7867: F, t8058: F, t8063: F, t8066: F, t8069: F, t8072: F, t8076: F, t8078: F, t8083: F, t913: F, t917: F, t930: F, t940: F, t953: F) -> F {
    let t8089 = F::new(0.5848048239485271795e1) * t940 * t8058 + F::new(0.11360101276506094136e1) * t913 * t8063 - F::new(0.60587206808032502059e1) * t8066 - F::new(0.75734008510040627575e0) * t8069 + F::new(0.33322963744417876133e2) * t8072 * t917 - F::new(0.57954409931925052365e-1) * t8076 + F::new(0.17386322979577515709e0) * t930 * t8078 + F::new(0.16121825426676543132e0) * t2704 * t2650 - F::new(0.20152281783345678915e-1) * t8083 + F::new(0.30228422675018518374e-1) * t953 * t7867 + F::new(0.46363527945540041892e0) * t2797 * t2701;
    t8089
}
