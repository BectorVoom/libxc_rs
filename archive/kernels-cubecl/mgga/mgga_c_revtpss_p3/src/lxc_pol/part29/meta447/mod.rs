//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta447 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1675;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1676;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta447<F: Float>(t676: F, t837: F, t2718: F, t867: F, t25372: F, t25410: F, t2408: F, t30: F, t605: F, t890: F, t2832: F, t2394: F, t33: F, t2411: F) -> (F, F, F, F, F, F, F, F) {
        let (t25412, t25416, t25431) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1675::<F>(t676, t837, t2718, t867, t25372, t25410);
        let (t25446, t25449, t25452, t25752, t25759) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1676::<F>(t2408, t30, t605, t890, t2832, t2394, t33, t2411);
    (t25412, t25416, t25431, t25446, t25449, t25452, t25752, t25759)
}
