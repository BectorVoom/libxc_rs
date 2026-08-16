//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta339 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1350;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta339(t3618: f64, t828: f64, t1209: f64, t3781: f64, t5330: f64, t1284: f64, t3555: f64, t3624: f64, t1121: f64, t3603: f64, t606: f64, t221: f64, t462: f64, t68: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t12787, t12808, t12809, t12831, t12832, t12840, t12851) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1350(t3618, t828, t1209, t3781, t5330, t1284, t3555, t3624, t1121, t3603, t606, t221, t462, t68);
    (t12787, t12808, t12809, t12831, t12832, t12840, t12851)
}
