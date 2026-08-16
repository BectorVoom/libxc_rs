//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta732 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2790;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2791;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta732<F: Float>(t212: F, t2237: F, t225: F, t816: F, t2665: F, t10689: F, t237: F, t247: F, t10709: F, t10744: F, t808: F, t2783: F, t9801: F, t10745: F, t2735: F, t4503: F, t10728: F, t10680: F, t2710: F, t2713: F, t10732: F, t10674: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t40488, t40489, t40507, t40509, t40517) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2790::<F>(t212, t2237, t225, t816, t2665, t10689, t237, t247, t10709, t10744, t808, t2783, t9801);
        let (t40518, t40521, t40523, t40526, t40529, t40532) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2791::<F>(t10745, t40517, t2735, t4503, t10728, t808, t10680, t2710, t2713, t10732, t10744, t10674);
    (t40488, t40489, t40507, t40509, t40517, t40518, t40521, t40523, t40526, t40529, t40532)
}
