//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 1050/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk1050(t14491: f64, t747: f64, t331: f64, t41574: f64, t41575: f64, t41579: f64, t41581: f64, t41585: f64, t41586: f64, t42906: f64, t42908: f64, t42910: f64, t42916: f64, t47080: f64, t47083: f64, t47085: f64, t47087: f64, t50808: f64, t50809: f64, t51000: f64, t51007: f64, t51013: f64, t51016: f64, t51029: f64, t51038: f64, t51047: f64, t51054: f64, t841: f64) -> f64 {
    let t51059 = t14491 * t747;
    let t51061 = -12.0_f64 * t47080 + 4.0_f64 * t47083 + t42906 + 4.0_f64 * t47085 + 4.0_f64 * t47087 + t41574 + t41575 - t50808 + t41579 - t41581 + (t51000 + t51007 + t51013 + t51016 + t51029 + t51038 + t51047 + t51054) * t331 - t51059 * t841 + t50809 - t42908 + t42910 - t42916 + t41585 + t41586;
    t51061
}
