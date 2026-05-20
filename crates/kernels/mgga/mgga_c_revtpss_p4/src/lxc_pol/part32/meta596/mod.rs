//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta596 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1928;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1929;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta596<F: Float>(t2470: F, t28313: F, t25387: F, t95822: F, t98892: F, t95537: F, t1957: F, t26550: F, t25372: F, t98801: F, t25386: F, t2471: F, t28373: F, t10867: F, t2061: F, t14481: F, t2062: F, t2782: F, t26519: F, t99257: F, t28341: F, t786: F, t789: F, t10073: F, t1579: F, t2066: F, t25390: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t103431, t103432, t103435, t103437, t103441, t103444, t103449) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1928::<F>(t2470, t28313, t25387, t95822, t98892, t95537, t1957, t26550, t25372, t98801, t25386, t2471, t28373);
        let (t103452, t103462, t103463, t103467, t103471) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1929::<F>(t10867, t2061, t14481, t2062, t2782, t26519, t99257, t28341, t786, t789, t10073, t1579, t2066, t25390);
    (t103431, t103432, t103435, t103437, t103441, t103444, t103449, t103452, t103462, t103463, t103467, t103471)
}
