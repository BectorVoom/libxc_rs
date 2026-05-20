//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta162 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk702;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk703;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta162<F: Float>(t225: F, t4376: F, t4407: F, t227: F, t73: F, t1544: F, t853: F, t775: F, t4343: F, t832: F, t1553: F, t1555: F, t229: F, t830: F, t833: F, t231: F) -> (F, F, F, F, F, F, F) {
        let (t4409, t4415, t4416, t4417, t4420, t4423) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk702::<F>(t225, t4376, t4407, t227, t73, t1544, t853, t775, t4343, t832, t1553, t1555, t229, t830, t833);
        let t4424 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk703::<F>(t231, t4423);
    (t4409, t4415, t4416, t4417, t4420, t4423, t4424)
}
