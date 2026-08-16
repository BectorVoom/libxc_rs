//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta497 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1878;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta497(t25997: f64, t4021: f64, t25273: f64, t533: f64, t816: f64, t540: f64, t7021: f64, t1372: f64, t3961: f64, t7252: f64, t1389: f64, t7269: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t25998, t26003, t26004, t26005, t26006, t26007, t26009) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1878(t25997, t4021, t25273, t533, t816, t540, t7021, t1372, t3961, t7252, t1389, t7269);
    (t25998, t26003, t26004, t26005, t26006, t26007, t26009)
}
