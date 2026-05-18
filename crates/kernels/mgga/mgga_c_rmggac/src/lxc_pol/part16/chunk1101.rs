//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1101/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1101<F: Float>(t35772: F, t37848: F, t37849: F, t37850: F, t40516: F, t40558: F, t40564: F, t40566: F, t43466: F, t43467: F, t43472: F, t47062: F, t47071: F, t47073: F, t47078: F, t47081: F, t47100: F, t47108: F) -> F {
    let t48877 = -F::new(0.9579393361491046851e0) * t40516 + F::new(0.5107751987195740728e-4) * t47062 - F::new(0.5107751987195740728e-4) * t47071 + F::new(0.5107751987195740728e-4) * t47073 - F::new(0.3192344991997337955e-4) * t47078 - F::new(0.30487649791575028312e-3) * t35772 - t37848 - t37849 + t37850 + F::new(0.49658699875514145967e-4) * t47081 - F::new(0.49658699875514145965e-4) * t40558 + t43466 - t43467 - F::new(0.49658699875514145965e-4) * t40564 + F::new(0.49658699875514145965e-4) * t40566 - t43472 + F::new(0.47896966807455234256e0) * t47100 + F::new(0.35922725105591425692e0) * t47108;
    t48877
}
