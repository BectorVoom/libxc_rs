//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1786/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1786(t556: f64, t786: f64, t9656: f64, t686: f64, t72: f64, t9658: f64, t10150: f64, t2435: f64, t9651: f64, t9680: f64, t1358: f64, t2439: f64, t4066: f64, t785: f64) -> (f64, f64, f64, f64) {
    let t47603 = t786 * t556 * t9656;
    let t47606 = t47603 * t9658 * t72 * t686;
    let t47608 = t2435 * t10150;
    let t47612 = t9680 * t9651 * t72 * t686;
    let t47616 = t2439 * t785 * t4066 * t1358;
    (t47606, t47608, t47612, t47616)
}
