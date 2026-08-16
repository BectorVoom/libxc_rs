//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta497 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1812;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1813;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta497(t25997: f64, t4021: f64, t25273: f64, t533: f64, t816: f64, t540: f64, t7021: f64, t1372: f64, t1389: f64, t7269: f64, t2736: f64, t2689: f64, t7256: f64, t2018: f64, t3951: f64, t807: f64, t1941: f64, t550: f64, t25240: f64, t3964: f64, t7262: f64, t820: f64, t843: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25998, t26003, t26004, t26005, t26009, t26011, t26012) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1812(t25997, t4021, t25273, t533, t816, t540, t7021, t1372, t1389, t7269, t2736, t2689, t7256);
        let (t26013, t26014, t26015, t26017, t26022, t26024) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1813(t26012, t2018, t3951, t807, t1941, t550, t1389, t25240, t3964, t7262, t820, t843);
    (t25998, t26003, t26004, t26005, t26009, t26011, t26013, t26014, t26015, t26017, t26022, t26024)
}
