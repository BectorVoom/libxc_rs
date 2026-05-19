//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1312/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1312<F: Float>(t24863: F, t24864: F, t30189: F, t30270: F, t49378: F, t49381: F, t49385: F, t49387: F, t49393: F, t56988: F, t56991: F, t56994: F, t56997: F, t56999: F) -> F {
    let t57416 = -F::new(0.375102e1) * t56988 + F::new(0.83356e0) * t56991 + F::new(0.125034e1) * t56994 + F::cast_from(0.12349037037037037037e1_f64) * t30189 + t24863 + t24864 - F::new(0.94674375e0) * t56997 + F::new(0.1262325e1) * t56999 + F::cast_from(0.12349037037037037037e0_f64) * t49378 + F::cast_from(0.27785333333333333333e0_f64) * t49381 + F::cast_from(0.21424148148148148148e1_f64) * t30270 - F::cast_from(0.27545333333333333332e1_f64) * t49385 + F::new(0.41318e1) * t49387 + F::cast_from(0.68863333333333333332e0_f64) * t49393;
    t57416
}
