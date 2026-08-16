//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta543 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2356;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta543<F: Float>(t12916: F, t5353: F, t3718: F, t5347: F, t3568: F, t471: F, t5351: F, t3720: F, t1781: F, t697: F, t1222: F, t5284: F, t73: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t17617, t17619, t17620, t17622, t17623, t17624, t17625, t17628, t17629, t17633) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2356::<F>(t12916, t5353, t3718, t5347, t3568, t471, t5351, t3720, t1781, t697, t1222, t5284, t73);
    (t17617, t17619, t17620, t17622, t17623, t17624, t17625, t17628, t17629, t17633)
}
