//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 935/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk935<F: Float>(t13914: F, t1955: F, t41576: F, t41579: F, t41581: F, t41585: F, t42908: F, t42910: F, t42912: F, t42914: F, t42916: F, t47065: F, t47066: F, t41586: F, t42470: F, t42473: F, t42475: F, t42481: F, t42483: F, t42485: F, t42917: F, t43346: F, t47070: F, t47071: F) -> (F, F) {
    let t47092 = -t13914 * t1955 + t41576 + t41579 - t41581 + t41585 - t42908 + t42910 - 6.0 * t42912 - t42914 - t42916 - t47065 - t47066;
    let t47095 = 2.0 * t42917 - t47070 + t41586 + t42470 + t47071 + t42473 + t43346 - t42475 + t42481 - t42483 + t42485;
    (t47092, t47095)
}
