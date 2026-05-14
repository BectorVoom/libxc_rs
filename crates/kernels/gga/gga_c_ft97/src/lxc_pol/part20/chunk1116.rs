//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1116/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1116<F: Float>(t1424: F, t42109: F, t13706: F, t1901: F, t10157: F, t1154: F, t2373: F, t24437: F, t6119: F, t1900: F, t6: F, t734: F, t91: F, t42123: F, t13757: F, t108179: F, t3712: F, t96934: F, t96935: F) -> (F, F, F, F, F) {
    let t109402 = t42109 * t1424;
    let t109404 = t1901 * t109402 * t13706;
    let t109409 = t24437 * t10157 * t6119 * t1154 * t2373;
    let t109414 = t91 * t734 * t6 * t1900;
    let t109415 = t42123 * t6119;
    let t109417 = t109414 * t109415 * t13757;
    let t109421 = t96934 * t96935 * t3712 * t108179;
    (t109404, t109409, t109414, t109417, t109421)
}
