//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta332 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1127;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1128;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta332<F: Float>(t4353: F, t9794: F, t10760: F, t10890: F, t1549: F, t10811: F, t4462: F, t4416: F, t808: F, t10886: F, t2703: F, t4458: F, t10769: F, t828: F, t1544: F, t836: F, t2746: F, t2710: F, t2713: F, t4371: F, t10744: F, t10905: F, t4442: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t14761, t14765, t14777, t14780, t14783) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1127::<F>(t4353, t9794, t10760, t10890, t1549, t10811, t4462, t4416, t808, t10886, t2703, t4458);
        let (t14785, t14786, t14791, t14817, t14820, t14823) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1128::<F>(t10769, t828, t1544, t836, t2746, t2710, t2713, t4371, t4353, t808, t10744, t10905, t4442);
    (t14761, t14765, t14777, t14780, t14783, t14785, t14786, t14791, t14817, t14820, t14823)
}
