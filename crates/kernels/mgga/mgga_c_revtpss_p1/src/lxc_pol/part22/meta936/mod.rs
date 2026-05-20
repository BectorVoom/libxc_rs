//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta936 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3169;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3170;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta936<F: Float>(t1256: F, t17333: F, t12268: F, t29054: F, t12898: F, t1786: F, t17202: F, t372: F, t17708: F, t45769: F, t44546: F, t5340: F, t5342: F, t11772: F, t17394: F, t3717: F, t12865: F, t17400: F, t1222: F, t1781: F, t2438: F, t12886: F, t5391: F, t12854: F, t21013: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t57604, t57606, t57615, t57621, t57631, t57635) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3169::<F>(t1256, t17333, t12268, t29054, t12898, t1786, t17202, t372, t17708, t45769, t44546, t5340, t5342);
        let (t57659, t57660, t57663, t57687, t57689, t57707) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3170::<F>(t11772, t17394, t3717, t12865, t17400, t1222, t1781, t2438, t12886, t5391, t12854, t21013);
    (t57604, t57606, t57615, t57621, t57631, t57635, t57659, t57660, t57663, t57687, t57689, t57707)
}
