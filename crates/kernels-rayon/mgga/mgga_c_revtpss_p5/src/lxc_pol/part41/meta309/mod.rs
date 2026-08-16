//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta309 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1077;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1078;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta309(t3634: f64, t828: f64, t3624: f64, t3746: f64, t3618: f64, t1209: f64, t3781: f64, t5330: f64, t1284: f64, t3555: f64, t1121: f64, t3603: f64, t606: f64, t221: f64, t462: f64, t68: f64, t461: f64, t3766: f64, t1214: f64, t11772: f64, t3623: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12772, t12784, t12787, t12809, t12832, t12839) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1077(t3634, t828, t3624, t3746, t3618, t1209, t3781, t5330, t1284, t3555, t1121, t3603);
        let (t12840, t12853, t12855, t12856, t12865) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1078(t12839, t606, t221, t462, t68, t461, t1209, t3766, t5330, t1214, t3603, t11772, t3623);
    (t12772, t12784, t12787, t12809, t12832, t12840, t12853, t12855, t12856, t12865)
}
