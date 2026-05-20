//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta504 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1792;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta504<F: Float>(t1955: F, t6041: F, t30: F, t6079: F, t1468: F, t1583: F, t6075: F, t33: F, t5966: F, t25759: F, t29598: F, t1544: F, t1711: F) -> (F, F, F, F, F, F, F) {
        let (t29698, t29713, t29716, t29719, t29939, t29946, t29949) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1792::<F>(t1955, t6041, t30, t6079, t1468, t1583, t6075, t33, t5966, t25759, t29598, t1544, t1711);
    (t29698, t29713, t29716, t29719, t29939, t29946, t29949)
}
