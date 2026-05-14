//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 647/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk647<F: Float>(t489: F, t57: F, t1620: F, t2215: F, t543: F, t108: F, t110: F, t548: F, t122: F, t2161: F, t625: F, t505: F, t2157: F, t1616: F, t560: F, t2201: F, t785: F) -> (F, F, F, F, F, F, F, F) {
    let t5119 = t57 * t489;
    let t5123 = t1620 * t2215;
    let t5132 = t543 * t543;
    let t5134 = t108 / t5132;
    let t5135 = t5134 * t110;
    let t5136 = t5135 * t548;
    let t5146 = t2161 * t122;
    let t5147 = t625 * t5146;
    let t5148 = t505 * t108;
    let t5149 = t5148 * t2157;
    let t5150 = t5147 * t5149;
    let t5177 = t1616 * t560;
    let t5179 = t2201 * t785 * t5177;
    (t5119, t5123, t5134, t5136, t5147, t5148, t5150, t5179)
}
