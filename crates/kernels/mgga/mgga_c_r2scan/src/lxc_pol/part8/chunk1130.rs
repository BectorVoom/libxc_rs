//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1130/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1130<F: Float>(t1399: F, t5764: F, t188: F, t1893: F, t390: F, t5446: F, t5448: F, t5768: F, t1906: F, t5598: F, t644: F, t5772: F, t5599: F, t5762: F, t652: F, t1647: F, t5381: F, t5388: F) -> (F, F, F, F, F, F, F, F) {
    let t21257 = 0.73692326405658170959e2 * t1399 * t5764;
    let t21262 = 0.44215395843394902576e3 * t390 * t5446 * t188 * t1893 * t5448;
    let t21264 = 0.45822920251607929917e1 * t1399 * t5768;
    let t21268 = 0.20620314113223568463e2 * t390 * t1906 * t644 * t5598;
    let t21270 = 0.28493333333333333333e0 * t1399 * t5772;
    let t21272 = 0.13746876075482378975e2 * t1399 * t5599;
    let t21276 = 0.41240628226447136925e2 * t390 * t5762 * t652 * t5448;
    let t21279 = 0.34737075717175875744e4 * t5388 * t5381 * t1647;
    (t21257, t21262, t21264, t21268, t21270, t21272, t21276, t21279)
}
