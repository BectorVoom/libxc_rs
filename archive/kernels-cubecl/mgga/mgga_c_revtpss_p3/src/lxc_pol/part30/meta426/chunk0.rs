//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1619/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1619<F: Float>(t1655: F, t697: F, t1011: F, t372: F, t4806: F, t15702: F, t15688: F, t3299: F, t1043: F, t905: F, t606: F, t3155: F) -> (F, F, F, F) {
    let t16219 = t697 * t1655;
    let t16220 = t1011 * t16219;
    let t16222 = t372 * t4806;
    let t16223 = t16222 * t15702;
    let t16226 = t3299 * t15688;
    let t16227 = t1043 * t905;
    let t16228 = t16227 * t606;
    let t16229 = t3155 * t16228;
    (t16220, t16223, t16226, t16229)
}
