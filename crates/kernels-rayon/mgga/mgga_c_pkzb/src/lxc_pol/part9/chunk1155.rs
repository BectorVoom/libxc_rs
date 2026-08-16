//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1155/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1155(t1034: f64, t5373: f64, t1721: f64, t16399: f64, t6908: f64, t1702: f64, t6930: f64, t1769: f64, t7005: f64, t1734: f64, t6859: f64, t164: f64, t1692: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20113 = t1034 * t5373;
    let t20114 = t20113 * t1721;
    let t20118 = t16399 * t6908;
    let t20121 = t1702 * t6930;
    let t20127 = t1769 * t7005;
    let t20137 = t6859 * t1734;
    let t20141 = t164 * t1692;
    (t20113, t20114, t20118, t20121, t20127, t20137, t20141)
}
