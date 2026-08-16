//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta191 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk807;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk808;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta191(t36: f64, t4186: f64, t70: f64, t1470: f64, t627: f64, t1486: f64, t607: f64, t1469: f64, t2275: f64, t606: f64, t48: f64, t2282: f64, t60: f64, t1474: f64, t1480: f64, t2290: f64, t44: f64, t56: f64, t614: f64, t620: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4187, t4188, t4191, t4196, t4201, t4202, t4205, t4210) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk807(t36, t4186, t70, t1470, t627, t1486, t607, t1469, t2275, t606, t48, t2282);
        let t4217 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk808(t4210, t606, t4186, t60, t1474, t1480, t2290, t4202, t4205, t44, t56, t614, t620);
    (t4187, t4188, t4191, t4196, t4201, t4202, t4205, t4210, t4217)
}
