//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3279/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3279(t22953: f64, t2661: f64, t3992: f64, t543: f64, t550: f64, t22857: f64, t46609: f64, t9994: f64, t4003: f64, t9934: f64, t221: f64, t22809: f64, t3978: f64, t3979: f64) -> (f64, f64, f64, f64, f64) {
    let t86203 = t2661 * t3992 * t550 * t22953 * t543;
    let t86205 = t550 * t22857;
    let t86208 = t2661 * t46609 * t86205 * t9994;
    let t86212 = t2661 * t9934 * t86205 * t4003;
    let t86220 = t3978 * t3979 * t221 * t22809;
    (t86203, t86205, t86208, t86212, t86220)
}
