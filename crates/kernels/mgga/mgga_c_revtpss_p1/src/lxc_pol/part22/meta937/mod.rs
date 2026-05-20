//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta937 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3171;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3172;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta937<F: Float>(t12808: F, t21013: F, t1222: F, t3698: F, t5047: F, t697: F, t12855: F, t12916: F, t17455: F, t16738: F, t17240: F, t16742: F, t16733: F, t12772: F, t17678: F, t5340: F, t17683: F, t5331: F, t12832: F, t17620: F, t17412: F, t3636: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t57710, t57726, t57735, t57743, t57746) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3171::<F>(t12808, t21013, t1222, t3698, t5047, t697, t12855, t12916, t17455, t16738, t17240, t16742);
        let (t57749, t57770, t57773, t57780, t57786) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3172::<F>(t1222, t16733, t17240, t12772, t17678, t5340, t17683, t5331, t12832, t17620, t17412, t3636);
    (t57710, t57726, t57735, t57743, t57746, t57749, t57770, t57773, t57780, t57786)
}
