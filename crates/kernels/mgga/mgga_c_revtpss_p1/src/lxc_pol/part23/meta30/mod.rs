//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta30 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk226;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk227;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk228;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk229;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk230;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta30<F: Float>(t20: F, t588: F, t12: F, t19: F, t2: F, t27: F, t21: F, t579: F, t25: F, t578: F, t582: F, t586: F, t88: F, t90: F, t29: F, t17: F, t4: F, t30: F, t33: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t590, t592, t594, t595, t596) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk226::<F>(t20, t588, t12, t19, t2, t27, t21, t579);
        let (t598, t599, t602) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk227::<F>(t25, t596, t578, t582, t586, t590, t594, t88, t90);
        let t603 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk228::<F>(t29, t602);
        let (t604, t605) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk229::<F>(t17, t2, t4);
        let t606 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk230::<F>(t30, t33, t605, zeta_threshold);
    (t590, t592, t594, t595, t596, t598, t599, t602, t603, t604, t605, t606)
}
