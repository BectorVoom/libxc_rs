//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1116/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1116(t14965: f64, t11739: f64, t11743: f64, t11747: f64, t11756: f64, t11770: f64, t11772: f64, t19977: f64, t19978: f64, t19979: f64, t19980: f64, t19981: f64, t19982: f64, t19984: f64, t19986: f64, t19988: f64, t19989: f64, t19990: f64) -> (f64, f64) {
    let t19991 = 48.0_f64 * t14965;
    let t19992 = -t19977 - t19978 + t11739 - t11743 + t19979 + t11747 - t19980 + t19981 - t11756 + t19982 - t19984 + t19986 - t19988 - t19989 - t19990 - t19991 + t11770 - t11772;
    (t19991, t19992)
}
