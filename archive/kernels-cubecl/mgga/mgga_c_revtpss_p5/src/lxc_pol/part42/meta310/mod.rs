//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta310 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1079;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1080;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta310<F: Float>(t3634: F, t828: F, t3624: F, t3746: F, t3618: F, t1209: F, t3781: F, t5330: F, t1284: F, t3555: F, t1121: F, t3603: F, t606: F, t221: F, t462: F, t68: F, t461: F, t3766: F, t1214: F, t11772: F, t3623: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t12772, t12784, t12787, t12809, t12832, t12839) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1079::<F>(t3634, t828, t3624, t3746, t3618, t1209, t3781, t5330, t1284, t3555, t1121, t3603);
        let (t12840, t12853, t12855, t12856, t12865) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1080::<F>(t12839, t606, t221, t462, t68, t461, t1209, t3766, t5330, t1214, t3603, t11772, t3623);
    (t12772, t12784, t12787, t12809, t12832, t12840, t12853, t12855, t12856, t12865)
}
