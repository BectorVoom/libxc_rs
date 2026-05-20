//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta206 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk842;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk843;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk844;
use chunk3::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk845;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta206<F: Float>(t1469: F, t2852: F, t606: F, t2850: F, t128: F, t2857: F, t904: F, t4186: F, t905: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t4573, t4574, t4575, t4576) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk842::<F>(t1469, t2852, t606, t2850, t128);
        let (t4578, t4579, t4580, t4581) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk843::<F>(t1469, t2857, t606, t904, t128);
        let t4583 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk844::<F>(t4186, t905);
        let (t4584, t4585) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk845::<F>(t4583, t904, t128);
    (t4573, t4574, t4575, t4576, t4578, t4579, t4580, t4581, t4583, t4584, t4585)
}
