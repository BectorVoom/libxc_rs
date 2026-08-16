//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta285 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1182;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta285<F: Float>(t547: F, t9646: F, t2236: F, t66: F, t240: F, t550: F, t268: F, t64: F, t8779: F, t159: F, t535: F, t65: F) -> (F, F, F, F, F, F, F) {
        let (t9720, t9721, t9723, t9725, t9727, t9729, t9731) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1182::<F>(t547, t9646, t2236, t66, t240, t550, t268, t64, t8779, t159, t535, t65);
    (t9720, t9721, t9723, t9725, t9727, t9729, t9731)
}
