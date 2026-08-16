//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta251 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk937;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk938;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk939;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta251<F: Float>(t4039: F, t4032: F, t4024: F, t3854: F, t3859: F, t3862: F, t3867: F, t3871: F, t3873: F, t4030: F, t4035: F, t4037: F, t4042: F, t225: F, t5638: F, t539: F, t73: F, t1412: F, t1868: F, t1353: F, t1394: F, t5591: F, t1392: F, t1395: F, t1877: F, t1879: F, t541: F, t543: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t5639, t5640, t5641, t5642) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk937::<F>(t4039, t4032, t4024, t3854, t3859, t3862, t3867, t3871, t3873, t4030, t4035, t4037, t4042);
        let (t5644, t5650, t5651, t5652, t5655, t5658) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk938::<F>(t225, t5638, t5642, t539, t73, t1412, t1868, t1353, t1394, t5591, t1392, t1395, t1877, t1879, t541);
        let t5659 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk939::<F>(t543, t5658);
    (t5639, t5640, t5641, t5644, t5650, t5651, t5652, t5655, t5658, t5659)
}
