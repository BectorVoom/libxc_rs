//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1600/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1600(t1131: f64, t1150: f64, t44036: f64, t44051: f64, t44067: f64, t44082: f64, t198: f64, t336: f64, t3801: f64, t43750: f64, t43757: f64, t43759: f64, t43761: f64, t43965: f64, t43970: f64, t43971: f64, t43980: f64, t43982: f64, t44011: f64, t44014: f64, t44021: f64) -> (f64, f64) {
    let t44087 = 1.0_f64 * t1131 * (t44036 + t44051 + t44067 + t44082) * t1150;
    let t44088 = -3.0_f64 * t198 * t336 * t3801 * t43971 - t43750 + t43757 - t43759 - t43761 - t43965 - t43970 - t43980 + t43982 + t44011 + t44014 - t44021 + t44087;
    (t44087, t44088)
}
