//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3347/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3347(t141: f64, t2908: f64, t63306: f64, t18908: f64, t2251: f64, t930: f64, t19006: f64, t698: f64, t51957: f64, t51963: f64, t60927: f64) -> (f64, f64, f64, f64, f64) {
    let t63311 = t141 * t2908 * t63306;
    let t63313 = t18908 * t2251;
    let t63315 = t141 * t930 * t63313;
    let t63320 = t698 * t19006;
    let t63325 = t51957 * t51963 * t60927;
    (t63311, t63313, t63315, t63320, t63325)
}
