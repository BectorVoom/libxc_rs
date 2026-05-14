//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1049/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1049<F: Float>(t469: F, t7954: F, t26017: F, t376: F, t5665: F, t1307: F, t38463: F, t8216: F, t1317: F, t1637: F, t6508: F, t26027: F, t375: F, t89: F, t25846: F, t358: F) -> (F, F, F, F, F, F, F, F, F) {
    let t100374 = t7954 * t469;
    let t100409 = t5665 * t376 * t26017;
    let t100410 = t100409 / 18.0;
    let t100411 = t38463 * t1307;
    let t100417 = t8216 * t1307;
    let t100427 = t1317 * t1637 * t6508;
    let t100430 = t89 * t375 * t26027;
    let t100431 = 2.0 / 9.0 * t100430;
    let t100440 = t25846 * t358;
    (t100374, t100409, t100410, t100411, t100417, t100427, t100430, t100431, t100440)
}
