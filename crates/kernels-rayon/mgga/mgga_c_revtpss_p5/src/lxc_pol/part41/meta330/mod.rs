//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta330 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1123;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1124;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta330(t10845: f64, t4430: f64, t1558: f64, t853: f64, t2749: f64, t2662: f64, t2661: f64, t4352: f64, t837: f64, t4416: f64, t221: f64, t2485: f64, t4424: f64, t2484: f64, t2652: f64, t4435: f64, t4343: f64, t854: f64, t236: f64, t807: f64, t4433: f64, t10703: f64, t2674: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14716, t14718, t14722, t14726, t14730, t14732) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1123(t10845, t4430, t1558, t853, t2749, t2662, t2661, t4352, t837, t4416, t221, t2485, t4424);
        let (t14734, t14736, t14744, t14759) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1124(t14732, t2484, t2652, t4435, t4343, t854, t236, t807, t221, t4433, t10703, t2674);
    (t14716, t14718, t14722, t14726, t14730, t14734, t14736, t14744, t14759)
}
