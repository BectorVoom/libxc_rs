//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta546 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1936;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1937;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1938;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta546<F: Float>(t30: F, t6079: F, t1468: F, t1583: F, t6075: F, t1940: F, t1963: F, t2403: F, t25206: F, t25445: F, t27368: F, t29592: F, t29599: F, t29602: F, t29606: F, t29705: F, t4541: F, t5824: F, t7091: F, t7749: F, t7783: F, t7787: F, t1651: F, t7810: F, t7145: F, t1976: F, t6392: F) -> (F, F, F, F, F, F, F) {
        let (t29713, t29716, t29719, t29726) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1936::<F>(t30, t6079, t1468, t1583, t6075, t1940, t1963, t2403, t25206, t25445, t27368, t29592, t29599, t29602, t29606, t29705, t4541, t5824, t7091, t7749, t7783, t7787);
        let t29727 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1937::<F>(t1651, t7810);
        let (t29728, t29731) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1938::<F>(t29727, t7145, t1976, t6392);
    (t29713, t29716, t29719, t29726, t29727, t29728, t29731)
}
