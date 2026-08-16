//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1133/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1133(t300: f64, t3650: f64, t779: f64, t2104: f64, t5974: f64, t9576: f64, t9571: f64, t5984: f64, t9307: f64, t17867: f64, t3646: f64, t9269: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25221 = t300 * t779 * t3650;
    let t25226 = t2104 * t5974 * t9576;
    let t25229 = t2104 * t5974 * t9571;
    let t25231 = t5984 * t9307;
    let t25236 = t2104 * t17867 * t3646;
    let t25239 = t2104 * t5974 * t9269;
    (t25221, t25226, t25229, t25231, t25236, t25239)
}
