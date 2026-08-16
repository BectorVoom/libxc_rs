//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta503 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1790;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1791;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta503<F: Float>(t25207: F, t29598: F, t1468: F, t1544: F, t30: F, t5962: F, t25262: F, t6024: F, t25270: F, t6037: F, t5980: F, t7038: F, t25237: F, t5989: F, t5993: F, t7045: F, t5985: F, t7025: F, t6019: F, t6030: F, t1558: F, t1579: F, t231: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t29599, t29602, t29606, t29616, t29618, t29620) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1790::<F>(t25207, t29598, t1468, t1544, t30, t5962, t25262, t6024, t25270, t6037, t5980, t7038);
        let (t29623, t29627, t29629, t29631, t29633, t29682) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1791::<F>(t25237, t5989, t5993, t7045, t5985, t7025, t6019, t7038, t6030, t1558, t1579, t231);
    (t29599, t29602, t29606, t29616, t29618, t29620, t29623, t29627, t29629, t29631, t29633, t29682)
}
