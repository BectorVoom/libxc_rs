//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta289 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1520;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1521;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta289<F: Float>(t3145: F, t334: F, t368: F, t365: F, t3144: F, t11240: F, t3153: F, t73: F, t3154: F, t1036: F, t357: F, t246: F, t676: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t11243, t11244, t11245, t11246, t11249) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1520::<F>(t3145, t334, t368, t365, t3144, t11240, t3153, t73);
        let (t11250, t11255, t11256, t11257, t11262) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1521::<F>(t11249, t3154, t1036, t11244, t11240, t357, t246, t676);
    (t11243, t11244, t11245, t11246, t11249, t11250, t11255, t11256, t11257, t11262)
}
