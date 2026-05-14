//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1165/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1165<F: Float>(t21248: F, t226: F, t5865: F, t5455: F, t721: F, t1399: F, t5764: F, t188: F, t1893: F, t390: F, t5446: F, t5448: F, t5768: F, t1906: F, t5598: F, t644: F) -> (F, F, F, F, F, F) {
    let t21251 = 0.14035736694323150897e2 * t5865 * t226 * t21248;
    let t21254 = 0.41558061853635513021e3 * t5455 * t721 * t21248;
    let t21257 = 0.73692326405658170959e2 * t1399 * t5764;
    let t21262 = 0.44215395843394902576e3 * t390 * t5446 * t188 * t1893 * t5448;
    let t21264 = 0.45822920251607929917e1 * t1399 * t5768;
    let t21268 = 0.20620314113223568463e2 * t390 * t1906 * t644 * t5598;
    (t21251, t21254, t21257, t21262, t21264, t21268)
}
