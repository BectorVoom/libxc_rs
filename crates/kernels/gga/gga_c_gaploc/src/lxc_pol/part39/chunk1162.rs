//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1162/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1162<F: Float>(t13937: F, t731: F, t43173: F, t43175: F, t43179: F, t43182: F, t43185: F, t43189: F, t43190: F, t43195: F, t43196: F, t43202: F) -> F {
    let t47702 = t731 * t13937;
    let t47704 = t43173 + F::new(0.92286314761706691403e-1) * t43175 - t43179 + t43182 + t43185 - t43189 - t43190 - t43195 + F::new(0.32043859292259267849e-3) * t43196 - F::new(0.42725145723012357132e-3) * t47702 - t43202;
    t47704
}
