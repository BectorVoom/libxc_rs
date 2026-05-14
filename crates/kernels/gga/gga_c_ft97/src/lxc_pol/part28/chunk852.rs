//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 852/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk852<F: Float>(t1882: F, t32459: F, t32500: F, t32613: F, t32555: F, t32504: F, t32471: F, t32547: F, t32479: F, t32542: F, t32581: F, t376: F, t89: F, t32568: F, t32627: F, t7283: F, t8232: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t137891 = t1882 * t32459;
    let t137900 = t1882 * t32500;
    let t137906 = t1882 * t32613;
    let t137908 = t1882 * t32555;
    let t137921 = t1882 * t32504;
    let t137923 = t1882 * t32471;
    let t137980 = t1882 * t32547;
    let t137987 = t1882 * t32479;
    let t137997 = t1882 * t32542;
    let t138000 = t89 * t376 * t32581;
    let t138029 = t1882 * t32568;
    let t138034 = t1882 * t32627;
    let t138057 = 4.0 / 27.0 * t8232 * t7283;
    (t137891, t137900, t137906, t137908, t137921, t137923, t137980, t137987, t137997, t138000, t138029, t138034, t138057)
}
