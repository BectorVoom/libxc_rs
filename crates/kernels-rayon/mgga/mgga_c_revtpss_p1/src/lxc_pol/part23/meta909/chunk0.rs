//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2919/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2919(t141: f64, t77579: f64, t930: f64, t2908: f64, t77584: f64, t11341: f64, t77564: f64, t77568: f64, t41294: f64, t77573: f64, t42731: f64, t52011: f64, t77513: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t77712 = t141 * t930 * t77579;
    let t77715 = t141 * t2908 * t77584;
    let t77718 = t141 * t11341 * t77564;
    let t77721 = t141 * t11341 * t77568;
    let t77724 = t141 * t41294 * t77573;
    let t77727 = t52011 * t42731 * t77513;
    (t77712, t77715, t77718, t77721, t77724, t77727)
}
