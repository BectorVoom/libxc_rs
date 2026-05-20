//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta217 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk963;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk964;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta217<F: Float>(t11132: F, t1034: F, t360: F, t11244: F, t11240: F, t3154: F, t357: F, t11249: F, t905: F, t3182: F, t828: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t11534, t11560, t11574, t11626, t11627, t11628, t11629, t11630, t11631) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk963::<F>(t11132, t1034, t360, t11244, t11240, t3154, t357);
        let (t11632, t11660, t11703) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk964::<F>(t11249, t11631, t3154, t905, t3182, t828);
    (t11534, t11560, t11574, t11626, t11627, t11628, t11629, t11630, t11631, t11632, t11660, t11703)
}
