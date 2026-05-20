//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta251 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1106;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1107;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1108;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1109;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1110;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta251<F: Float>(t187: F, t6800: F, t5636: F, t2522: F, t2562: F, t2579: F, t2587: F, t3871: F, t3873: F, t4027: F, t6780: F, t6802: F, t2569: F, t3854: F, t3859: F, t3862: F, t3865: F, t3867: F, t4035: F, t4037: F, t4042: F, t6777: F, t6778: F, t6779: F, t225: F, t1868: F, t4049: F, t1394: F, t6816: F, t1877: F, t1879: F, t539: F, t541: F, t543: F) -> (F, F, F, F, F, F, F, F) {
        let (t6827, t6828, t6829) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1106::<F>(t187, t6800, t5636, t2522, t2562, t2579, t2587, t3871, t3873, t4027, t6780, t6802);
        let t6830 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1107::<F>(t2569, t3854, t3859, t3862, t3865, t3867, t4035, t4037, t4042, t6777, t6778, t6779);
        let (t6832, t6836) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1108::<F>(t225, t6829, t6830, t1868);
        let (t6837, t6840, t6843) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1109::<F>(t4049, t6836, t1394, t6816, t1877, t1879, t539, t541, t6832);
        let t6844 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1110::<F>(t543, t6843);
    (t6827, t6828, t6832, t6836, t6837, t6840, t6843, t6844)
}
