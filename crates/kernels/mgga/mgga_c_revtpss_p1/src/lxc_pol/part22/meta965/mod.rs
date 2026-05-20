//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta965 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3228;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta965<F: Float>(t40121: F, t50058: F, t40127: F, t40132: F, t18263: F, t2414: F, t40207: F, t6002: F, t40139: F, t50084: F, t14353: F, t14365: F, t18871: F, t2403: F, t40131: F, t40137: F, t4433: F, t4541: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t61214, t61215, t61219, t61220, t61222, t61224, t61225, t61229, t61230) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3228::<F>(t40121, t50058, t40127, t40132, t18263, t2414, t40207, t6002, t40139, t50084, t14353, t14365, t18871, t2403, t40131, t40137, t4433, t4541);
    (t61214, t61215, t61219, t61220, t61222, t61224, t61225, t61229, t61230)
}
