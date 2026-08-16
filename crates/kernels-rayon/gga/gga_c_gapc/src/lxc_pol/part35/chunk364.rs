//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 364/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk364(t1672: f64, t505: f64, t1671: f64, t632: f64, t668: f64, t457: f64, t1665: f64, t604: f64, t624: f64, t189: f64, t190: f64, t195: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1673 = t1672 * t505;
    let t1674 = t1671 * t1673;
    let t1677 = t632 * t668;
    let t1678 = t1672 * t457;
    let t1679 = t1665 * t1678;
    let t1682 = t604 * t624;
    let t1686 = t189 * t190 * t195;
    (t1673, t1674, t1677, t1678, t1679, t1682, t1686)
}
