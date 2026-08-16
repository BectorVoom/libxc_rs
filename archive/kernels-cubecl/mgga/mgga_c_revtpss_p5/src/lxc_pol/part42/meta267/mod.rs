//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta267 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1013;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1014;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta267<F: Float>(t547: F, t9646: F, t2236: F, t66: F, t240: F, t550: F, t268: F, t64: F, t8779: F, t159: F, t535: F, t65: F, t235: F, t1389: F, t3964: F, t2735: F, t546: F, t1353: F, t1412: F, t808: F, t1369: F, t2699: F, t1372: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t9720, t9721, t9725, t9727, t9729, t9731) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1013::<F>(t547, t9646, t2236, t66, t240, t550, t268, t64, t8779, t159, t535, t65);
        let (t9732, t9735, t9736, t9739, t9741, t9742) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1014::<F>(t235, t9731, t1389, t3964, t2735, t546, t1353, t1412, t808, t1369, t2699, t1372);
    (t9720, t9721, t9725, t9727, t9729, t9731, t9732, t9735, t9736, t9739, t9741, t9742)
}
