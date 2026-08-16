//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta327 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1330;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta327(t11865: f64, t3090: f64, t3316: f64, t994: f64, t4891: f64, t1016: f64, t697: f64, t1011: f64, t11132: f64, t126: f64, t373: f64, t828: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11866, t11874, t11875, t11880, t11881, t11890, t11921, t11922) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1330(t11865, t3090, t3316, t994, t4891, t1016, t697, t1011, t11132, t126, t373, t828);
    (t11866, t11874, t11875, t11880, t11881, t11890, t11921, t11922)
}
