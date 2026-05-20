//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2802/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2802<F: Float>(t231: F, t2782: F, t2783: F, t51380: F, t10073: F, t14504: F, t10547: F, t14568: F, t50560: F, t2797: F, t18632: F, t836: F) -> (F, F, F, F, F) {
    let t51519 = t2782 * t2783 * t51380 * t231;
    let t51521 = t10073 * t14504;
    let t51522 = F::cast_from(0.19514881078765566038e-2_f64) * t51521;
    let t51523 = t14568 * t10547;
    let t51525 = t50560 * t231;
    let t51527 = t2782 * t2797 * t51525;
    let t51529 = t18632 * t836;
    (t51519, t51522, t51523, t51527, t51529)
}
