//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1748/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1748<F: Float>(t448: F, t90305: F, t90317: F, t300: F, t24480: F, t5192: F, t6438: F, t44091: F, t44093: F, t16840: F, t24221: F, t1150: F, t12248: F) -> (F, F, F, F, F, F, F) {
    let t90319 = (t90305 + t90317) * t448;
    let t90321 = F::cast_from(0.19751673498613801407e-1_f64) * t300 * t90319;
    let t90323 = F::cast_from(0.14035736694323150897e2_f64) * t5192 * t24480;
    let t90324 = t6438 * t6438;
    let t90327 = F::cast_from(0.24955700379505800916e5_f64) * t44091 * t90324 * t44093;
    let t90329 = F::cast_from(24.0_f64) * t16840 * t24221;
    let t90332 = F::cast_from(24.0_f64) * t12248 * t90324 * t1150;
    (t90319, t90321, t90323, t90324, t90327, t90329, t90332)
}
