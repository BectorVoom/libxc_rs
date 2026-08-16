//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta252 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk942;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk943;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk944;
use chunk3::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk945;
use chunk4::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk946;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta252<F: Float>(t1390: F, t5659: F, t828: F, t1883: F, t221: F, t4019: F, t4018: F, t241: F, t4000: F, t820: F, t550: F, t72: F, t245: F, t125: F, t1882: F, t1398: F, t4003: F, t1388: F, t1410: F, t3931: F, t3956: F, t4022: F, t4064: F, t5606: F, t5611: F, t5614: F, t5619: F, t5623: F, t5625: F, t5629: F) -> (F, F, F, F, F, F, F, F) {
        let (t5661, t5665, t5666, t5671, t5672) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk942::<F>(t1390, t5659, t828, t1883, t221, t4019, t4018, t241, t4000, t820, t550, t72);
        let t5673 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk943::<F>(t245, t5672);
        let t5674 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk944::<F>(t125, t1882);
        let t5675 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk945::<F>(t1398, t4003);
        let (t5677, t5680) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk946::<F>(t5674, t5675, t5673, t1388, t1410, t3931, t3956, t4022, t4064, t5606, t5611, t5614, t5619, t5623, t5625, t5629, t5661, t5666, t5671);
    (t5661, t5665, t5671, t5673, t5674, t5675, t5677, t5680)
}
