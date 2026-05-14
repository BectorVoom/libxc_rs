//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 901/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk901<F: Float>(t14491: F, t747: F, t331: F, t41574: F, t41575: F, t41579: F, t41581: F, t41585: F, t41586: F, t42906: F, t42908: F, t42910: F, t42916: F, t47080: F, t47083: F, t47085: F, t47087: F, t50808: F, t50809: F, t51000: F, t51007: F, t51013: F, t51016: F, t51029: F, t51038: F, t51047: F, t51054: F, t841: F) -> (F,) {
    let t51059 = t14491 * t747;
    let t51061 = -12.0 * t47080 + 4.0 * t47083 + t42906 + 4.0 * t47085 + 4.0 * t47087 + t41574 + t41575 - t50808 + t41579 - t41581 + (t51000 + t51007 + t51013 + t51016 + t51029 + t51038 + t51047 + t51054) * t331 - t51059 * t841 + t50809 - t42908 + t42910 - t42916 + t41585 + t41586;
    (t51061,)
}
