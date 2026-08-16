//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 857/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk857<F: Float>(t1035: F, t8515: F, t1983: F, t378: F, t7150: F, t8521: F, t995: F, t342: F, t7135: F, t1071: F, t3140: F, t1078: F, t1982: F) -> (F, F, F, F, F) {
    let t25604 = t8515 * t1035;
    let t25605 = t1983 * t25604;
    let t25610 = t7150 * t378;
    let t25611 = t25610 * t8521;
    let t25629 = t995 * t8521;
    let t25634 = t342 * t7135;
    let t25638 = t1071 * t3140;
    let t25640 = t1982 * t25638 * t1078;
    (t25605, t25611, t25629, t25634, t25640)
}
