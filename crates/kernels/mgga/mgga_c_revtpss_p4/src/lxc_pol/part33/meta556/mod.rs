//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta556 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1947;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta556<F: Float>(t33: F, t5966: F, t1963: F, t25759: F, t29598: F, t1544: F, t1711: F, t5962: F, t6079: F, t1583: F, t6075: F, t1940: F, t2403: F, t25206: F, t25445: F, t27368: F, t29705: F, t4541: F, t6416: F, t7091: F, t7783: F, t7862: F, t7869: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t29939, t29940, t29946, t29949, t29953, t29964, t29967, t29970, t29977) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1947::<F>(t33, t5966, t1963, t25759, t29598, t1544, t1711, t5962, t6079, t1583, t6075, t1940, t2403, t25206, t25445, t27368, t29705, t4541, t6416, t7091, t7783, t7862, t7869);
    (t29939, t29940, t29946, t29949, t29953, t29964, t29967, t29970, t29977)
}
