//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1509/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1509<F: Float>(t23253: F, t40348: F, t10777: F, t10779: F, t1559: F, t5984: F, t10905: F, t23275: F, t6035: F, t61956: F, t40725: F, t5988: F) -> (F, F, F, F, F) {
    let t76647 = t40348 * t23253;
    let t76672 = t10777 * t10779 * t5984 * t1559;
    let t76677 = t10905 * t23275;
    let t76689 = t10777 * t10779 * t61956 * t6035;
    let t76701 = t10777 * t40725 * t5988 * t1559;
    (t76647, t76672, t76677, t76689, t76701)
}
