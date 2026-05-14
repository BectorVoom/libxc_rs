//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1264/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1264<F: Float>(t31928: F, t9307: F, t31910: F, t9311: F, t31921: F, t9320: F, t2932: F, t31920: F, t31924: F, t111259: F, t31902: F, t111081: F, t111416: F, t111418: F, t111421: F, t15445: F, t2679: F, t9310: F) -> (F, F) {
    let t111423 = t31928 * t9307;
    let t111425 = t9311 * t31910;
    let t111427 = t31921 * t9320;
    let t111429 = t31921 * t9307;
    let t111432 = t2932 * t31920 * t9307;
    let t111434 = t31924 * t9320;
    let t111436 = t31902 * t111259;
    let t111439 = 0.31250000000000000001e-1 * t111416 + 0.31250000000000000001e-1 * t111418 + 0.120625e-1 * t111421 - 0.14583333333333333334e0 * t111423 - 0.62500000000000000002e-1 * t111425 + 0.24305555555555555556e0 * t111427 + 0.24305555555555555556e0 * t111429 + 0.93819444444444444446e-1 * t111432 - 0.14583333333333333334e0 * t111434 - 0.62500000000000000002e-1 * t111436 + 0.92858888888888888885e-1 * t111081;
    let t111446 = t15445 * t9310 * t2679;
    (t111439, t111446)
}
