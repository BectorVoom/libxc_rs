//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta353 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1164;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta353(t1160: f64, t5117: f64, t1737: f64, t3476: f64, t16868: f64, t16712: f64, t16892: f64, t16708: f64, t1179: f64, t5155: f64, t1719: f64, t3383: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17026, t17032, t17050, t17052, t17066, t17075, t17089, t17092) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1164(t1160, t5117, t1737, t3476, t16868, t16712, t16892, t16708, t1179, t5155, t1719, t3383);
    (t17026, t17032, t17050, t17052, t17066, t17075, t17089, t17092)
}
