//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta332 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1113;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1114;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta332<F: Float>(t12808: F, t5330: F, t3153: F, t3601: F, t1284: F, t3555: F, t3624: F, t221: F, t462: F, t68: F, t461: F, t1209: F, t3766: F, t11772: F, t3623: F, t3717: F, t1263: F, t675: F, t1122: F, t247: F, t1261: F, t126: F, t3617: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t12809, t12810, t12832, t12853, t12854) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1113::<F>(t12808, t5330, t3153, t3601, t1284, t3555, t3624, t221, t462, t68, t461, t1209, t3766);
        let (t12855, t12865, t12866, t12879, t12882, t12884) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1114::<F>(t12854, t5330, t11772, t3623, t3717, t1263, t675, t1122, t247, t1261, t126, t3617);
    (t12809, t12810, t12832, t12853, t12855, t12865, t12866, t12879, t12882, t12884)
}
