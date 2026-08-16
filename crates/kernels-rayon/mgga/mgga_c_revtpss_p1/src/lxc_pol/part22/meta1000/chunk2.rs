//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3400/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3400(t63700: f64, t63715: f64, t63731: f64, t63747: f64, t63764: f64, t63780: f64, t63797: f64, t63813: f64, t964: f64, t973: f64, t981: f64, t11465: f64, t3015: f64, t6205: f64) -> (f64, f64, f64) {
    let t63816 = t63700 + t63715 + t63731 + t63747 + t63764 + t63780 + t63797 + t63813;
    let t63820 = 0.5848223622634646207e0_f64 * t981 * t964 * t63816 * t973;
    let t63826 = 0.10389515463408878255e3_f64 * t981 * t11465 * t6205 * t3015;
    (t63816, t63820, t63826)
}
