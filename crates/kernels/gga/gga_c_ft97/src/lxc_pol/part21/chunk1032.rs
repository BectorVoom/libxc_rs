//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1032/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1032<F: Float>(t5675: F, t8216: F, t5617: F, t7241: F, t1570: F, t1307: F, t7763: F, t1317: F, t1637: F, t5684: F, t1557: F, t1322: F, t2999: F, t89: F, t1636: F, t5700: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t93379 = t8216 * t5675;
    let t93392 = t7241 * t5617;
    let t93409 = t5617 * t1570;
    let t93416 = t1307 * t7763;
    let t93424 = t1317 * t1637 * t5684;
    let t93425 = 2.0 / 9.0 * t93424;
    let t93434 = t5617 * t1557;
    let t93452 = t89 * t2999 * t1322;
    let t93453 = 28.0 / 81.0 * t93452;
    let t93458 = t89 * t1636 * t5700;
    (t93379, t93392, t93409, t93416, t93424, t93425, t93434, t93452, t93453, t93458)
}
