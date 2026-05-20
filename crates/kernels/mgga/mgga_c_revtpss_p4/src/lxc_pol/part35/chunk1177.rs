//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1177/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1177<F: Float>(t2118: F, t6936: F, t1913: F, t8130: F, t2110: F, t6951: F, t30626: F, t575: F, t1921: F, t8113: F, t30663: F, t571: F) -> (F, F, F, F, F, F) {
    let t111405 = t6936 * t2118;
    let t111408 = t1913 * t8130;
    let t111410 = t2110 * t6951;
    let t111411 = t30626 * t575;
    let t111412 = t8113 * t1921;
    let t111415 = t571 * t30663;
    (t111405, t111408, t111410, t111411, t111412, t111415)
}
