//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta555 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1945;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1946;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta555<F: Float>(t30: F, t6079: F, t1468: F, t1583: F, t6075: F, t1940: F, t1963: F, t2403: F, t25206: F, t25445: F, t27368: F, t29592: F, t29599: F, t29602: F, t29606: F, t29705: F, t4541: F, t5824: F, t7091: F, t7749: F, t7783: F, t7787: F, t5966: F, t1544: F, t198: F, t207: F, t29598: F, t29704: F, t5962: F, t892: F) -> (F, F, F, F, F, F) {
        let (t29713, t29716, t29719, t29726) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1945::<F>(t30, t6079, t1468, t1583, t6075, t1940, t1963, t2403, t25206, t25445, t27368, t29592, t29599, t29602, t29606, t29705, t4541, t5824, t7091, t7749, t7783, t7787);
        let (t29907, t29930) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1946::<F>(t1963, t5966, t1544, t1583, t1940, t198, t207, t2403, t25445, t27368, t29598, t29704, t4541, t5962, t6075, t6079, t7091, t7783, t892);
    (t29713, t29716, t29719, t29726, t29907, t29930)
}
