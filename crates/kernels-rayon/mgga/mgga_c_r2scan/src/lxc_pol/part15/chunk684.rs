//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 684/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk684(t5142: f64, t551: f64, t2184: f64, t122: f64, t2161: f64, t625: f64, t108: f64, t505: f64, t2157: f64, t1234: f64, t788: f64, t2207: f64, t785: f64) -> (f64, f64, f64, f64, f64) {
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
