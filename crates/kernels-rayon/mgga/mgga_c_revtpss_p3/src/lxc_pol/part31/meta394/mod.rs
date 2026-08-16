//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta394 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1435;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta394(t1749: f64, t3520: f64, t16868: f64, t16712: f64, t16892: f64, t16708: f64, t3495: f64, t1770: f64, t3781: f64, t1284: f64, t1811: f64, t1209: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17097, t17115, t17117, t17131, t17140, t17154, t17183, t17192) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1435(t1749, t3520, t16868, t16712, t16892, t16708, t3495, t1770, t3781, t1284, t1811, t1209);
    (t17097, t17115, t17117, t17131, t17140, t17154, t17183, t17192)
}
