//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1724/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1724(t1386: f64, t2482: f64, t2668: f64, t9821: f64, t13999: f64, t9842: f64, t9828: f64, t9962: f64, t124: f64, t3923: f64, t3938: f64, t9816: f64, t9818: f64) -> (f64, f64, f64, f64, f64) {
    let t46740 = t2482 * t1386 * t2668;
    let t46741 = t46740 * t9821;
    let t46747 = t13999 * t9842;
    let t46749 = t9962 * t9828;
    let t46751 = t124 * t3923;
    let t46754 = t9816 * t9818 * t46751 * t3938;
    (t46741, t46747, t46749, t46751, t46754)
}
