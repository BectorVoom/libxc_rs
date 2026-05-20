//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2593/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2593<F: Float>(t10368: F, t56: F, t1518: F, t670: F, t1921: F, t5789: F, t1913: F, t5808: F, t22532: F, t575: F, t21661: F, t602: F) -> (F, F, F, F, F, F) {
    let t60311 = t56 * t10368;
    let t60595 = t670 * t1518;
    let t60620 = t5789 * t1921;
    let t60624 = t1913 * t5808;
    let t60629 = t22532 * t575;
    let t60670 = t21661 * t602;
    (t60311, t60595, t60620, t60624, t60629, t60670)
}
