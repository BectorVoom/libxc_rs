//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2300/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2300<F: Float>(t12895: F, t12971: F, t193: F, t202: F, t2522: F, t2553: F, t262: F, t4314: F, t46481: F, t47149: F, t47151: F, t47153: F, t47156: F, t47159: F, t47161: F, t47162: F, t47164: F, t47564: F, t47593: F, t47631: F, t776: F, t870: F) -> F {
    let t47644 = t193 * t202 * (t46481 + t47564 + t47593 + t47631) * t870 + t47149 + t47151 + t47153 + F::cast_from(9.0_f64) * t2522 * t12895 * t2553 + t47156 + t47159 + t47161 + t47162 + F::cast_from(18.0_f64) * t4314 * t262 * t12971 * t776 + t47164;
    t47644
}
