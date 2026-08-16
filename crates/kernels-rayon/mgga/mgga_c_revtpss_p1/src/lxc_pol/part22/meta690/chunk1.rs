//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2690/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2690(t22046: f64, t3936: f64, t9835: f64, t1414: f64, t21969: f64, t828: f64, t221: f64, t3979: f64, t6816: f64, t3978: f64, t3989: f64, t6880: f64) -> (f64, f64, f64, f64, f64) {
    let t22048 = t3936 * t22046 * t9835;
    let t22052 = t1414 * t828 * t21969;
    let t22056 = t3979 * t221 * t6816;
    let t22057 = t3978 * t22056;
    let t22059 = t3989 * t6880;
    (t22048, t22052, t22056, t22057, t22059)
}
