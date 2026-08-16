//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta709 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2732;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2733;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2734;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta709<F: Float>(t26: F, t65: F, t1868: F, t4147: F, t1651: F, t385: F, t1774: F, t494: F, t9163: F, t99: F, t107: F, t9232: F, t5672: F, t828: F, t4363: F, t2565: F, t702: F, t9305: F, t2576: F, t2585: F, t9274: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t33127, t33596, t33754, t34934, t36227, t36415) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2732::<F>(t26, t65, t1868, t4147, t1651, t385, t1774, t494, t9163, t99, t107, t9232);
        let (t36776, t36833, t39419) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2733::<F>(t5672, t828, t4363, t2565, t702, t9305);
        let t39422 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2734::<F>(t2576, t2585, t9274);
    (t33127, t33596, t33754, t34934, t36227, t36415, t36776, t36833, t39419, t39422)
}
