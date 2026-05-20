//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta206 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk940;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk941;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta206<F: Float>(t10696: F, t240: F, t72: F, t136: F, t2476: F, t2482: F, t596: F, t849: F, t2681: F, t820: F, t2719: F, t2735: F, t2783: F, t810: F, t9784: F, t9789: F, t235: F, t2453: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t10697, t10698, t10703, t10716, t10722, t10726, t10744) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk940::<F>(t10696, t240, t72, t136, t2476, t2482, t596, t849, t2681, t820, t2719, t2735, t2783);
        let (t10756, t10758, t10759, t10760) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk941::<F>(t810, t9784, t9789, t235, t2783, t2453);
    (t10697, t10698, t10703, t10716, t10722, t10726, t10744, t10756, t10758, t10759, t10760)
}
