//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta30 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk193;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk194;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk195;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta30<F: Float>(t15: F, t580: F, t14: F, t2: F, t11: F, t22: F, t21: F, t3: F, t20: F, t12: F, t19: F, t27: F, t579: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t582, t583) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk193::<F>(t15, t580, t14, t2);
        let (t584, t586, t587, t588) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk194::<F>(t11, t583, t22, t21, t3);
        let (t590, t592, t594, t595, t596) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk195::<F>(t20, t588, t12, t19, t2, t27, t21, t579);
    (t582, t583, t584, t586, t587, t588, t590, t592, t594, t595, t596)
}
