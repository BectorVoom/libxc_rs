//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2778/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2778(t2661: f64, t3992: f64, t48533: f64, t6869: f64, t14045: f64, t22096: f64, t21990: f64, t5608: f64, t9934: f64, t1413: f64, t46835: f64, t74483: f64) -> (f64, f64, f64, f64) {
    let t74598 = t2661 * t3992 * t48533 * t6869;
    let t74602 = t2661 * t3992 * t14045 * t22096;
    let t74606 = t2661 * t9934 * t5608 * t21990;
    let t74638 = t46835 * t1413 * t74483;
    (t74598, t74602, t74606, t74638)
}
