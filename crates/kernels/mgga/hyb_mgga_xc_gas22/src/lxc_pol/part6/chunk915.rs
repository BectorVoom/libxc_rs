//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 915/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk915<F: Float>(t2014: F, t3283: F, t684: F, t1318: F, t763: F, t675: F, t2002: F, t3282: F, t2028: F, t1243: F, t6469: F, t1240: F, t2011: F, t3124: F, t7884: F, t2024: F, t2027: F, t3288: F, t6471: F, t6474: F, t6477: F, t6481: F, t677: F, t687: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8560 = t684 * t2014 * t3283 / 96.0;
    let t8561 = t763 * t1318;
    let t8562 = t8561 * t675;
    let t8566 = t3282 * t2002;
    let t8570 = t3282 * t2028;
    let t8575 = t684 * t6469 * t1243;
    let t8577 = t1240 * t2011;
    let t8579 = t7884 * t3124;
    let t8583 = t6471 / 144.0 - t6474 / 96.0 - t6477 / 192.0 - t6481 / 144.0 - t8560 - t684 * t687 * t8562 / 32.0 - t684 * t687 * t8566 / 64.0 - t2024 * t2027 * t8570 / 48.0 + t8575 / 288.0 + t8577 / 96.0 - 7.0 / 32.0 * t8579 - 3.0 / 32.0 * t677 * t3288;
    (t8560, t8561, t8562, t8566, t8570, t8575, t8577, t8579, t8583)
}
