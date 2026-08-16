//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta669 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2195;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2196;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta669<F: Float>(t4173: F, t4187: F, t21698: F, t603: F, t5816: F, t640: F, t77: F, t29561: F, t644: F, t4241: F, t7705: F, t1927: F, t21804: F, t76: F, t2242: F, t5819: F, t38: F, t60670: F, t13272: F, t1470: F, t29543: F, t1497: F, t7719: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t108813, t108816, t108864, t108872, t108876, t108879) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2195::<F>(t4173, t4187, t21698, t603, t5816, t640, t77, t29561, t644, t4241, t7705, t1927);
        let (t108941, t108945, t108952, t108966, t108975, t108978) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2196::<F>(t21804, t76, t2242, t5819, t38, t60670, t13272, t1470, t29543, t644, t77, t1497, t7719);
    (t108813, t108816, t108864, t108872, t108876, t108879, t108941, t108945, t108952, t108966, t108975, t108978)
}
