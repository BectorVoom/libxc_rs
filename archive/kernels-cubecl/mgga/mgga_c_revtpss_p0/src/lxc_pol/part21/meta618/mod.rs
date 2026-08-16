//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta618 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2372;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2373;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta618<F: Float>(t10115: F, t225: F, t880: F, t10866: F, t232: F, t235: F, t2723: F, t10666: F, t221: F, t2484: F, t2485: F, t2482: F, t2719: F, t596: F, t10852: F, t10858: F, t10863: F, t10868: F, t820: F, t843: F, t10874: F, t27: F, t10872: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t40317, t40318, t40321, t40322, t40325, t40333, t40336) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2372::<F>(t10115, t225, t880, t10866, t232, t235, t2723, t10666, t221, t2484, t2485, t2482, t2719, t596);
        let (t40337, t40345, t40349, t40355) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2373::<F>(t10852, t40336, t10858, t10863, t10868, t820, t843, t10874, t2482, t27, t10872, t221, t2485);
    (t40317, t40318, t40321, t40322, t40325, t40333, t40337, t40345, t40349, t40355)
}
