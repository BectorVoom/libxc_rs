//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1246/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1246<F: Float>(t109414: F, t109415: F, t17785: F, t30970: F, t684: F, t96934: F, t96935: F, t18123: F, t2354: F, t6118: F, t6119: F, t108120: F, t17790: F, t1901: F, t109402: F, t17794: F) -> (F, F, F, F, F) {
    let t123941 = t109414 * t109415 * t17785;
    let t123945 = t96934 * t96935 * t30970 * t684;
    let t123949 = t6118 * t2354 * t6119 * t18123;
    let t123952 = t1901 * t108120 * t17790;
    let t123955 = t1901 * t109402 * t17794;
    (t123941, t123945, t123949, t123952, t123955)
}
