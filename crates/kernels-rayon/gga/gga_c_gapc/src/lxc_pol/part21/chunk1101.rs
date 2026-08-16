//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 1101/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk1101(t18679: f64, t2763: f64, t3699: f64, t7730: f64, t1899: f64, t277: f64, t33666: f64, t26597: f64, t2660: f64, t16182: f64, t102: f64, t9281: f64) -> (f64, f64, f64, f64) {
    let t33670 = t3699 * t18679 * t2763 * t7730;
    let t33671 = t277 * t1899 * t33666 * t33670;
    let t33673 = t2660 * t26597;
    let t33674 = t33673 * t16182;
    let t33676 = t9281 * t102;
    (t33671, t33673, t33674, t33676)
}
