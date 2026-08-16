//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta853 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2995;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2996;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta853(t14426: f64, t72: f64, t757: f64, t14616: f64, t2619: f64, t14386: f64, t2615: f64, t198: f64, t775: f64, t10565: f64, t1469: f64, t706: f64, t1531: f64, t36: f64, t14362: f64, t9863: f64, t9866: f64, t2609: f64, t4395: f64, t14341: f64, t2398: f64, t13312: f64, t750: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t49986, t50047, t50058, t50080, t50084) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2995(t14426, t72, t757, t14616, t2619, t14386, t2615, t198, t775, t10565, t1469, t706);
        let (t50089, t50092, t50094, t50097, t50099, t50113) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2996(t1531, t36, t14362, t9863, t9866, t2609, t4395, t14341, t2398, t13312, t706, t750);
    (t49986, t50047, t50058, t50080, t50084, t50089, t50092, t50094, t50097, t50099, t50113)
}
