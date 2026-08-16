//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta168 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk717;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk718;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk719;
use chunk3::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk720;
use chunk4::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk721;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta168<F: Float>(t1469: F, t2857: F, t606: F, t904: F, t128: F, t4186: F, t905: F, t2847: F, t2848: F, t4571: F, t4576: F, t291: F, t1596: F, t914: F, t936: F, t1610: F, t2869: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t4578, t4579) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk717::<F>(t1469, t2857, t606);
        let (t4580, t4581) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk718::<F>(t4579, t904, t128);
        let t4583 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk719::<F>(t4186, t905);
        let (t4584, t4585) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk720::<F>(t4583, t904, t128);
        let (t4587, t4589, t4590, t4592, t4594) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk721::<F>(t2847, t2848, t4571, t4576, t4581, t4585, t291, t1596, t914, t936, t1610, t2869);
    (t4578, t4579, t4580, t4581, t4583, t4584, t4585, t4587, t4589, t4590, t4592, t4594)
}
