//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta768 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2721;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2722;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta768(t50058: f64, t40125: f64, t40127: f64, t40132: f64, t2408: f64, t775: f64, t40139: f64, t11075: f64, t14318: f64, t14436: f64, t14468: f64, t2403: f64, t2430: f64, t262: f64, t40131: f64, t40137: f64, t4433: f64, t4541: f64, t198: f64, t10565: f64, t1469: f64, t706: f64, t1531: f64, t36: f64, t10440: f64, t14362: f64, t9863: f64, t9866: f64, t40143: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t50059, t50063, t50064, t50065, t50070, t50078) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2721(t50058, t40125, t40127, t40132, t2408, t775, t40139, t11075, t14318, t14436, t14468, t2403, t2430, t262, t40131, t40137, t4433, t4541);
        let (t50080, t50085, t50091, t50093, t50095, t50096) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2722(t198, t775, t10565, t1469, t706, t1531, t36, t10440, t14362, t9863, t9866, t40143);
    (t50059, t50063, t50064, t50065, t50070, t50078, t50080, t50085, t50091, t50093, t50095, t50096)
}
