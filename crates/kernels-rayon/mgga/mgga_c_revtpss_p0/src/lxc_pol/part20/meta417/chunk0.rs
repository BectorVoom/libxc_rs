//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1559/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1559(t3361: f64, t39443: f64, t141: f64, t43764: f64, t1146: f64, t9303: f64, t12270: f64, t698: f64, t2304: f64, t12254: f64, t2439: f64, t3424: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t43765 = t3361 * t3361;
    let t43766 = 1.0_f64 / t43765;
    let t43767 = t43766 * t39443;
    let t43769 = t141 * t43764 * t43767;
    let t43771 = t9303 * t1146;
    let t43773 = t698 * t12270;
    let t43776 = 1.0_f64 / t3361 / t2304;
    let t43777 = t43776 * t39443;
    let t43779 = t141 * t12254 * t43777;
    let t43781 = t2439 * t3424;
    (t43766, t43767, t43769, t43771, t43773, t43776, t43777, t43779, t43781)
}
