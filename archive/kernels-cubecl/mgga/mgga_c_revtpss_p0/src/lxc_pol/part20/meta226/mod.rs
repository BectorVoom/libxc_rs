//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta226 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1016;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1017;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta226<F: Float>(t10709: F, t2662: F, t2661: F, t2652: F, t2656: F, t2482: F, t596: F, t849: F, t2677: F, t2665: F, t9775: F, t2681: F, t820: F, t857: F, t10673: F, t10676: F, t10678: F, t10682: F, t10687: F, t10692: F, t10693: F, t10700: F, t10706: F, t851: F) -> (F, F, F, F) {
        let (t10710, t10711, t10713, t10716, t10717, t10719, t10722) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1016::<F>(t10709, t2662, t2661, t2652, t2656, t2482, t596, t849, t2677, t2665, t9775, t2681, t820);
        let t10725 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1017::<F>(t10722, t857, t10673, t10676, t10678, t10682, t10687, t10692, t10693, t10700, t10706, t10711, t10713, t10717, t10719, t851);
    (t10710, t10716, t10722, t10725)
}
