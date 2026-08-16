//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2014/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2014(t1544: f64, t836: f64, t2749: f64, t14785: f64, t2746: f64, t828: f64) -> (f64, f64, f64, f64) {
    let t14786 = t1544 * t836;
    let t14787 = t14786 * t2749;
    let t14788 = t14785 * t14787;
    let t14791 = t2746 * t828;
    (t14786, t14787, t14788, t14791)
}
