//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1163/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1163(t4185: f64, t840: f64, t14423: f64, t875: f64, t13796: f64, t3989: f64, t1133: f64, t898: f64, t13798: f64, t3214: f64, t3959: f64, t14121: f64, t3209: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14718 = t840 * t4185;
    let t14720 = t14423 * t875;
    let t14721 = t13796 * t14720;
    let t14722 = t3989 * t14721;
    let t14724 = t898 * t1133;
    let t14725 = t14724 * t13798;
    let t14726 = t13796 * t14725;
    let t14727 = t3989 * t14726;
    let t14729 = t3959 * t3214;
    let t14731 = t14121 * t3209;
    (t14718, t14721, t14722, t14724, t14726, t14727, t14729, t14731)
}
