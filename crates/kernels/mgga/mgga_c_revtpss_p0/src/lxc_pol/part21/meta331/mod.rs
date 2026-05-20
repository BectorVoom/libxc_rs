//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta331 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1638;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1639;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta331<F: Float>(t11247: F, t373: F, t3153: F, t73: F, t3154: F, t1042: F, t1036: F, t11244: F, t11240: F, t357: F, t246: F, t676: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t11248, t11249) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1638::<F>(t11247, t373, t3153, t73);
        let (t11250, t11251, t11252, t11255, t11256, t11257, t11258, t11259, t11262) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1639::<F>(t11249, t3154, t11248, t1042, t1036, t11244, t11240, t357, t246, t676);
    (t11248, t11249, t11250, t11251, t11252, t11255, t11256, t11257, t11258, t11259, t11262)
}
