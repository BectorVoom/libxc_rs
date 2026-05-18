//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 268/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk268<F: Float>(t92: F, t93: F, t6: F, t8: F, t191: F, t56: F, t209: F, t371: F, t763: F) -> (F, F, F, F) {
    let t1006 = F::new(1.0) / t92 * t93;
    let t1011 = t6 * t8;
    let t1013 = t191 * t56;
    let t1023 = t209 * t763 * t371;
    (t1006, t1011, t1013, t1023)
}
