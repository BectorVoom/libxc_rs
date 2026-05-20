//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta749 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2625;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2626;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta749<F: Float>(t1857: F, t9855: F, t9410: F, t9413: F, t47081: F, t5571: F, t9372: F, t13597: F, t2496: F, t123: F, t2630: F, t5566: F, t13665: F, t9863: F, t9866: F, t47093: F, t39989: F, t47084: F, t47086: F, t47088: F, t47092: F, t47096: F, t47098: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t48291, t48293, t48295, t48296, t48298, t48300, t48302) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2625::<F>(t1857, t9855, t9410, t9413, t47081, t5571, t9372, t13597, t2496, t123, t2630, t5566);
        let (t48303, t48305, t48307, t48308, t48309) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2626::<F>(t48302, t13665, t9863, t9866, t47093, t39989, t47084, t47086, t47088, t47092, t47096, t47098, t48291, t48293, t48295, t48296, t48298, t48300);
    (t48291, t48293, t48295, t48296, t48298, t48300, t48303, t48305, t48307, t48308, t48309)
}
