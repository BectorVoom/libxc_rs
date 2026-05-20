//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta305 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1561;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1562;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1563;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta305<F: Float>(t10703: F, t221: F, t2394: F, t2674: F, t231: F, t243: F, t2645: F, t2662: F, t2661: F, t2652: F, t2656: F, t2482: F, t596: F, t849: F, t2677: F, t2665: F, t9775: F, t2681: F, t820: F, t857: F, t10673: F, t10676: F, t10678: F, t10682: F, t10687: F, t10692: F, t10693: F, t10700: F, t851: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t10705, t10706, t10709, t10710, t10711, t10713, t10716) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1561::<F>(t10703, t221, t2394, t2674, t231, t243, t2645, t2662, t2661, t2652, t2656, t2482, t596, t849);
        let (t10717, t10719, t10722) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1562::<F>(t10716, t2677, t2665, t9775, t2681, t820, t849);
        let (t10723, t10725) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1563::<F>(t10722, t857, t10673, t10676, t10678, t10682, t10687, t10692, t10693, t10700, t10706, t10711, t10713, t10717, t10719, t851);
    (t10705, t10706, t10709, t10710, t10711, t10713, t10716, t10717, t10719, t10722, t10723, t10725)
}
