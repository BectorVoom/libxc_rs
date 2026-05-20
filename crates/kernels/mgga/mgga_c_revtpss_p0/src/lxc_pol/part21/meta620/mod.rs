//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta620 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2376;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2377;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta620<F: Float>(t10815: F, t2648: F, t2756: F, t2681: F, t2719: F, t820: F, t2726: F, t10850: F, t10861: F, t221: F, t2485: F, t10111: F, t823: F, t9720: F, t685: F, t827: F, t837: F, t10837: F, t9775: F, t10828: F, t2741: F, t10818: F, t10703: F, t2674: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t40393, t40395, t40399, t40403, t40406) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2376::<F>(t10815, t2648, t2756, t2681, t2719, t820, t2726, t10850, t10861, t221, t2485, t10111, t823, t9720);
        let (t40409, t40411, t40413, t40421) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2377::<F>(t40406, t685, t827, t837, t10837, t9775, t10828, t2741, t10818, t221, t10703, t2674);
    (t40393, t40395, t40399, t40403, t40406, t40409, t40411, t40413, t40421)
}
