//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 638/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk638<F: Float>(t13749: F, t493: F, t492: F, t105: F, t169: F, t172: F, t452: F, t12032: F, t921: F, t2355: F, t3718: F, t1382: F, t123: F, t3689: F, t883: F, t912: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t13750 = t493 * t13749;
    let t13751 = t492 * t13750;
    let t13753 = 0.28455006635676149599e-1 * t105 * t13751;
    let t13755 = t13749 * t169 * t172;
    let t13756 = t452 * t13755;
    let t13758 = 0.28455006635676149599e-1 * t105 * t13756;
    let t13762 = t12032 * t921;
    let t13764 = t2355 * t3718;
    let t13765 = t3718 * t921;
    let t13766 = t1382 * t13765;
    let t13777 = t3689 * t123;
    let t13778 = t13777 * t883;
    let t13779 = t912 * t13778;
    (t13750, t13751, t13753, t13755, t13756, t13758, t13762, t13764, t13765, t13766, t13777, t13778, t13779)
}
