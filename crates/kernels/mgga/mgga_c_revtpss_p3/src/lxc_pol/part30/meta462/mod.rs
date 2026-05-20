//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta462 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1758;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1759;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta462<F: Float>(t2035: F, t25188: F, t531: F, t7311: F, t7238: F, t2014: F, t7312: F, t7315: F, t2394: F, t30: F, t1962: F, t198: F, t206: F, t2411: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t25189, t25190, t25191, t25193, t25194, t25196, t25198, t25206) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1758::<F>(t2035, t25188, t531, t7311, t7238, t2014, t7312, t7315, t2394, t30, t1962, t198, t206);
        let t25207 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1759::<F>(t2411, t30);
    (t25189, t25190, t25191, t25193, t25194, t25196, t25198, t25206, t25207)
}
