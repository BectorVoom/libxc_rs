//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1558/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1558(t1196: f64, t12555: f64, t43752: f64, t43753: f64, t12564: f64, t3531: f64, t12571: f64, t3543: f64, t12258: f64, t698: f64, t13026: f64, t240: f64) -> (f64, f64, f64, f64, f64) {
    let t43757 = 0.12304822629859687989e5_f64 * t1196 * t43752 * t43753 * t12555;
    let t43759 = 0.23392894490538584828e1_f64 * t3531 * t12564;
    let t43761 = 0.10389515463408878255e3_f64 * t12571 * t3543;
    let t43762 = t698 * t12258;
    let t43764 = t240 * t13026;
    (t43757, t43759, t43761, t43762, t43764)
}
