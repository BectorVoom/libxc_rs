//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 982/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk982<F: Float>(t43173: F, t43175: F, t43179: F, t43182: F, t43185: F, t43189: F, t43190: F, t43195: F, t43196: F, t43202: F, t47702: F, t1897: F, t1901: F, t47322: F, t13921: F, t7137: F) -> (F, F, F) {
    let t47704 = t43173 + 0.92286314761706691403e-1 * t43175 - t43179 + t43182 + t43185 - t43189 - t43190 - t43195 + 0.32043859292259267849e-3 * t43196 - 0.42725145723012357132e-3 * t47702 - t43202;
    let t47708 = 0.76905262301422242837e-2 * t1897 * t1901 * t47322;
    let t47709 = t7137 * t13921;
    (t47704, t47708, t47709)
}
