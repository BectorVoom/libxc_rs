//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1022/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1022<F: Float>(t102980: F, t93377: F, t2435: F, t8011: F, t25431: F, t2439: F, t93170: F, t93190: F, t10073: F, t26554: F, t27198: F, t15003: F, t95773: F, t26506: F, t27216: F, t786: F, t7998: F, t867: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t102981 = t93377 * t102980;
    let t102993 = t8011 * t2435;
    let t102994 = t25431 * t102993;
    let t103000 = t8011 * t2439;
    let t103001 = t93170 * t103000;
    let t103009 = t93190 * t102980;
    let t103017 = t10073 * t27198 * t26554;
    let t103030 = t95773 * t15003;
    let t103063 = t27216 * t26506;
    let t103067 = t786 * t7998 * t867;
    (t102981, t102993, t102994, t103000, t103001, t103009, t103017, t103030, t103063, t103067)
}
