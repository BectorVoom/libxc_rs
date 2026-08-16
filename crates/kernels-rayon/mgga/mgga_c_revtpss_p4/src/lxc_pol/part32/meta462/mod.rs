//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta462 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1685;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1686;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta462(t2018: f64, t3951: f64, t807: f64, t1941: f64, t550: f64, t1389: f64, t25240: f64, t3964: f64, t7262: f64, t820: f64, t843: f64, t1401: f64, t241: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t26014, t26016, t26017, t26021, t26024) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1685(t2018, t3951, t807, t1941, t550, t1389, t25240, t3964, t7262, t820, t843);
        let (t26025, t26028) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1686(t1401, t26024, t241, t7262, t820);
    (t26014, t26016, t26017, t26021, t26024, t26025, t26028)
}
