//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta768 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2721;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2722;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta768<F: Float>(t50058: F, t40125: F, t40127: F, t40132: F, t2408: F, t775: F, t40139: F, t11075: F, t14318: F, t14436: F, t14468: F, t2403: F, t2430: F, t262: F, t40131: F, t40137: F, t4433: F, t4541: F, t198: F, t10565: F, t1469: F, t706: F, t1531: F, t36: F, t10440: F, t14362: F, t9863: F, t9866: F, t40143: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t50059, t50063, t50064, t50065, t50070, t50078) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2721::<F>(t50058, t40125, t40127, t40132, t2408, t775, t40139, t11075, t14318, t14436, t14468, t2403, t2430, t262, t40131, t40137, t4433, t4541);
        let (t50080, t50085, t50091, t50093, t50095, t50096) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2722::<F>(t198, t775, t10565, t1469, t706, t1531, t36, t10440, t14362, t9863, t9866, t40143);
    (t50059, t50063, t50064, t50065, t50070, t50078, t50080, t50085, t50091, t50093, t50095, t50096)
}
