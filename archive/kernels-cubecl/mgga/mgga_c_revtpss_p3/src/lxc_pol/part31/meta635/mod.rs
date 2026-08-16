//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta635 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2089;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2090;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta635<F: Float>(t16060: F, t7111: F, t25539: F, t4924: F, t16219: F, t139: F, t27526: F, t3252: F, t4574: F, t1014: F, t4579: F, t1035: F, t27543: F, t7150: F, t99708: F, t1977: F, t994: F, t11627: F, t1983: F, t99682: F, t11223: F, t7143: F, t3057: F, t7810: F, t11120: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t100359, t100363, t100365, t100370, t100398, t100431) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2089::<F>(t16060, t7111, t25539, t4924, t16219, t139, t27526, t3252, t4574, t1014, t4579, t1035, t27543);
        let (t100494, t100586, t100596, t100658, t100681, t100690) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2090::<F>(t7150, t99708, t1977, t994, t11627, t1983, t99682, t11223, t7143, t3057, t7810, t11120);
    (t100359, t100363, t100365, t100370, t100398, t100431, t100494, t100586, t100596, t100658, t100681, t100690)
}
