//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta853 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2995;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2996;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta853<F: Float>(t14426: F, t72: F, t757: F, t14616: F, t2619: F, t14386: F, t2615: F, t198: F, t775: F, t10565: F, t1469: F, t706: F, t1531: F, t36: F, t14362: F, t9863: F, t9866: F, t2609: F, t4395: F, t14341: F, t2398: F, t13312: F, t750: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t49986, t50047, t50058, t50080, t50084) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2995::<F>(t14426, t72, t757, t14616, t2619, t14386, t2615, t198, t775, t10565, t1469, t706);
        let (t50089, t50092, t50094, t50097, t50099, t50113) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2996::<F>(t1531, t36, t14362, t9863, t9866, t2609, t4395, t14341, t2398, t13312, t706, t750);
    (t49986, t50047, t50058, t50080, t50084, t50089, t50092, t50094, t50097, t50099, t50113)
}
