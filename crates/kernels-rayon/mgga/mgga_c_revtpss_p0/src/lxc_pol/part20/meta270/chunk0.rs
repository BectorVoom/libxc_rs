//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1119/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1119(t3182: f64, t828: f64, t2852: f64, t357: f64, t2251: f64, t3093: f64, t3109: f64) -> (f64, f64, f64, f64, f64) {
    let t11703 = t828 * t3182;
    let t11704 = t357 * t2852;
    let t11705 = t11704 * t2251;
    let t11706 = t3093 * t11705;
    let t11707 = t11703 * t11706;
    let t11710 = t828 * t3109;
    (t11703, t11705, t11706, t11707, t11710)
}
