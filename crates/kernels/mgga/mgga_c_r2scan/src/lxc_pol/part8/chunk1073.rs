//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1073/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1073<F: Float>(t406: F, t4879: F, t2332: F, t1762: F, t4705: F, t4776: F, t1409: F, t1497: F, t234: F, t453: F, t1384: F, t18904: F, t4859: F, t1380: F, t14: F, t1464: F, t1481: F, t18938: F, t4870: F) -> (F, F, F, F, F, F, F, F) {
    let t19014 = t406 * t4879;
    let t19025 = t2332 * t2332;
    let t19026 = 1.0 / t19025;
    let t19032 = 0.19263893255070628432e1 * t1762 * t4776 * t4705;
    let t19033 = t1409 * t1409;
    let t19037 = 0.35089341735807877242e1 * t234 * t1497 * t19033 * t453;
    let t19041 = 0.6233709278045326953e3 * t234 * t4859 * t18904 * t1384;
    let t19048 = 0.51947577317044391277e2 * t234 * t1380 * t19033 * t1384;
    let t19057 = 0.62071215503128080361e4 * t14 / t1481 / t1464 * t18938 * t4870;
    (t19014, t19026, t19032, t19033, t19037, t19041, t19048, t19057)
}
