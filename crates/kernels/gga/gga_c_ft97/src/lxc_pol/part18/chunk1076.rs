//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1076/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1076<F: Float>(t1882: F, t23145: F, t23249: F, t47667: F, t23254: F, t8392: F, t23258: F, t487: F, t5617: F, t23203: F, t23346: F, t1328: F, t7943: F, t89: F, t23241: F, t1637: F, t5706: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t91537 = t1882 * t23145;
    let t91539 = t47667 * t23249;
    let t91543 = t8392 * t23254;
    let t91565 = t8392 * t23258;
    let t91583 = t487 * t5617;
    let t91605 = t1882 * t23203;
    let t91614 = t8392 * t23346;
    let t91625 = 28.0 / 81.0 * t89 * t7943 * t1328;
    let t91626 = t8392 * t23241;
    let t91629 = t89 * t1637 * t5706;
    (t91537, t91539, t91543, t91565, t91583, t91605, t91614, t91625, t91626, t91629)
}
