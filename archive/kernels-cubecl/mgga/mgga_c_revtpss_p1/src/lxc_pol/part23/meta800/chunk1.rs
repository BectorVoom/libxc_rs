//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2627/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2627<F: Float>(t10777: F, t10779: F, t50412: F, t6035: F, t4321: F, t4534: F, t689: F, t10995: F, t18312: F, t686: F, t72: F, t18804: F, t2470: F) -> (F, F, F, F) {
    let t62502 = t10777 * t10779 * t50412 * t6035;
    let t62516 = t689 * t4321 * t4534;
    let t62523 = t10995 * t18312 * t72 * t686;
    let t62528 = t10995 * t18804 * t2470;
    (t62502, t62516, t62523, t62528)
}
