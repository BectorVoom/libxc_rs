//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1033/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1033<F: Float>(t1497: F, t7719: F, t1927: F, t5872: F, t2247: F, t5826: F, t108138: F, t96187: F, t96236: F, t30256: F, t689: F, t25904: F, t25899: F, t30278: F, t686: F, t72: F) -> (F, F, F, F, F, F, F, F) {
    let t108978 = t7719 * t1497;
    let t108986 = t1927 * t5872;
    let t108990 = t2247 * t5826;
    let t109391 = t96187 * t108138;
    let t109393 = t96236 * t108138;
    let t109396 = t30256 * t689;
    let t109397 = t25904 * t109396;
    let t109400 = t25899 * t109396;
    let t109403 = t30278 * t72 * t686;
    (t108978, t108986, t108990, t109391, t109393, t109397, t109400, t109403)
}
