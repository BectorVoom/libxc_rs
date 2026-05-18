//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 684/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk684<F: Float>(t5142: F, t551: F, t2184: F, t122: F, t2161: F, t625: F, t108: F, t505: F, t2157: F, t1234: F, t788: F, t2207: F, t785: F) -> (F, F, F, F, F) {
    let t5143 = t551 * t5142;
    let t5144 = t2184 * t5143;
    let t5146 = t2161 * t122;
    let t5147 = t625 * t5146;
    let t5148 = t505 * t108;
    let t5149 = t5148 * t2157;
    let t5150 = t5147 * t5149;
    let t5162 = t788 * t1234;
    let t5164 = t2207 * t785 * t5162;
    (t5144, t5147, t5148, t5150, t5164)
}
