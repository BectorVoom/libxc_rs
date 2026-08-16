//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta368 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1401;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta368(t14732: f64, t2484: f64, t2652: f64, t4435: f64, t4343: f64, t854: f64, t236: f64, t807: f64, t221: f64, t4433: f64, t10703: f64, t2674: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t14734, t14736, t14741, t14744, t14756, t14757, t14759) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1401(t14732, t2484, t2652, t4435, t4343, t854, t236, t807, t221, t4433, t10703, t2674);
    (t14734, t14736, t14741, t14744, t14756, t14757, t14759)
}
