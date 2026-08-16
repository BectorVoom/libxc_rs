//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1925/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1925(t1398: f64, t1868: f64, t3938: f64, t13783: f64, t3935: f64, t828: f64) -> (f64, f64, f64, f64) {
    let t13784 = t1868 * t1398;
    let t13785 = t13784 * t3938;
    let t13786 = t13783 * t13785;
    let t13789 = t3935 * t828;
    (t13784, t13785, t13786, t13789)
}
