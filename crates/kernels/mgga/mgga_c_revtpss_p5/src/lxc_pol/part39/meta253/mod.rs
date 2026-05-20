//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta253 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk945;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk946;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk947;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta253<F: Float>(t1873: F, t3957: F, t1353: F, t1872: F, t800: F, t124: F, t5591: F, t3938: F, t5674: F, t3936: F, t1399: F, t5673: F, t125: F, t1868: F, t1370: F, t3934: F, t3944: F, t3950: F, t3953: F, t3958: F, t3967: F, t3976: F, t3982: F, t3987: F, t3990: F, t3996: F, t5680: F) -> (F, F, F, F, F, F, F, F) {
        let (t5681, t5686, t5689, t5690, t5697, t5701) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk945::<F>(t1873, t3957, t1353, t1872, t800, t124, t5591, t3938, t5674, t3936, t1399, t5673);
        let (t5704, t5706, t5709) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk946::<F>(t125, t1868, t1399, t3936, t1370, t3934, t3944, t3950, t3953, t3958, t3967, t3976, t3982, t3987, t3990, t3996, t5681, t5686, t5690, t5697, t5701);
        let t5710 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk947::<F>(t5680, t5709);
    (t5686, t5689, t5690, t5697, t5701, t5704, t5706, t5710)
}
