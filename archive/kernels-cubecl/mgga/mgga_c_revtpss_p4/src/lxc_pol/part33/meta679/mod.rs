//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta679 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2212;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2213;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta679<F: Float>(t109269: F, t28199: F, t25082: F, t27153: F, t33651: F, t6941: F, t7331: F, t5795: F, t7950: F, t7953: F, t1916: F, t28265: F, t28277: F, t1518: F, t572: F, t670: F, t7741: F, t28280: F, t1459: F, t30191: F, t28264: F, t5920: F, t105886: F, t117: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t109271, t109274, t109282, t109288, t109291, t109293) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2212::<F>(t109269, t28199, t25082, t27153, t33651, t6941, t7331, t5795, t7950, t7953, t1916, t28265);
        let (t109295, t109299, t109305, t109307, t109310, t109315) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2213::<F>(t1916, t28277, t1518, t572, t670, t7741, t28280, t1459, t30191, t28264, t5920, t105886, t117);
    (t109271, t109274, t109282, t109288, t109291, t109293, t109295, t109299, t109305, t109307, t109310, t109315)
}
