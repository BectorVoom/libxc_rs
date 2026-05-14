//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 650/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk650<F: Float>(t543: F, t108: F, t110: F, t548: F, t1632: F, t2185: F, t551: F, t2184: F, t122: F, t2161: F, t625: F, t505: F, t2157: F, t1234: F, t788: F, t2207: F, t785: F) -> (F, F, F, F, F, F, F, F) {
    let t5132 = t543 * t543;
    let t5134 = t108 / t5132;
    let t5135 = t5134 * t110;
    let t5136 = t5135 * t548;
    let t5142 = t1632 * t2185;
    let t5143 = t551 * t5142;
    let t5144 = t2184 * t5143;
    let t5146 = t2161 * t122;
    let t5147 = t625 * t5146;
    let t5148 = t505 * t108;
    let t5149 = t5148 * t2157;
    let t5150 = t5147 * t5149;
    let t5162 = t788 * t1234;
    let t5164 = t2207 * t785 * t5162;
    (t5134, t5136, t5142, t5144, t5147, t5148, t5150, t5164)
}
