//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta249 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1082;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta249<F: Float>(t11249: F, t3154: F, t11248: F, t1042: F, t1036: F, t11244: F, t11240: F, t357: F, t246: F, t676: F) -> (F, F, F, F, F, F, F) {
        let (t11251, t11252, t11255, t11256, t11258, t11259, t11262) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1082::<F>(t11249, t3154, t11248, t1042, t1036, t11244, t11240, t357, t246, t676);
    (t11251, t11252, t11255, t11256, t11258, t11259, t11262)
}
