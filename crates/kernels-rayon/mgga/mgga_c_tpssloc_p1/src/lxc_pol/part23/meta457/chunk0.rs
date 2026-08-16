//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1331/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1331(t75852: f64, t75862: f64, t75875: f64, t75891: f64, t75934: f64, t75947: f64, t76543: f64, t76556: f64, t41666: f64, t75836: f64, t123: f64, t41664: f64) -> (f64, f64, f64) {
    let t76559 = t75852 + t75862 + t75875 + t75891 + t75934 + t75947 + t76543 + t76556;
    let t76572 = t41666 * t75836;
    let t76574 = t123 * t41664 * t76572;
    (t76559, t76572, t76574)
}
