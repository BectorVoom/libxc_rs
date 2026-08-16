//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2690/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2690(t11922: f64, t16067: f64, t19721: f64, t19566: f64, t3090: f64, t1086: f64, t19462: f64, t19972: f64, t4892: f64, t19658: f64, t3124: f64, t19882: f64, t3106: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t67526 = t16067 * t11922 * t19721;
    let t67528 = t19566 * t3090;
    let t67551 = t19462 * t1086 * t3090;
    let t67560 = t4892 * t11922 * t19972;
    let t67568 = t3124 * t19658;
    let t67571 = t3106 * t19882;
    (t67526, t67528, t67551, t67560, t67568, t67571)
}
