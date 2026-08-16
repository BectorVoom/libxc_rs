//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2739/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2739<F: Float>(t17609: F, t5265: F, t17544: F, t5274: F, t1222: F, t17471: F, t20298: F, t20302: F, t1260: F, t57465: F, t21334: F, t17763: F, t5378: F) -> (F, F, F, F, F, F, F) {
    let t71550 = t17609 * t5265;
    let t71552 = t5274 * t17544;
    let t71571 = t1222 * t17471 * t20298;
    let t71582 = t1222 * t17471 * t20302;
    let t71585 = t57465 * t1260;
    let t71590 = t21334 * t1260;
    let t71598 = t17763 * t5378;
    (t71550, t71552, t71571, t71582, t71585, t71590, t71598)
}
