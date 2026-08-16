//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1097/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1097<F: Float>(t13914: F, t1955: F, t41576: F, t41579: F, t41581: F, t41585: F, t42908: F, t42910: F, t42912: F, t42914: F, t42916: F, t47065: F, t47066: F) -> F {
    let t47092 = -t13914 * t1955 + t41576 + t41579 - t41581 + t41585 - t42908 + t42910 - F::cast_from(6.0_f64) * t42912 - t42914 - t42916 - t47065 - t47066;
    t47092
}
