//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2682/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2682(t1011: f64, t15993: f64, t18937: f64, t127: f64, t15700: f64, t19979: f64, t19981: f64, t11859: f64, t11922: f64, t19635: f64, t11875: f64, t19640: f64) -> (f64, f64, f64, f64) {
    let t66822 = t1011 * t15993 * t18937;
    let t66860 = t15700 * t127 * t19979 * t19981;
    let t66943 = t11859 * t11922 * t19635;
    let t66951 = t11875 * t11922 * t19640;
    (t66822, t66860, t66943, t66951)
}
