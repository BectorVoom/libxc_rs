//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta277 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1494;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1495;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1496;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta277<F: Float>(t10716: F, t2677: F, t2665: F, t9775: F, t2681: F, t820: F, t849: F, t857: F, t240: F, t2719: F, t2735: F, t2783: F) -> (F, F, F, F, F, F) {
        let (t10717, t10719, t10722) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1494::<F>(t10716, t2677, t2665, t9775, t2681, t820, t849);
        let (t10723, t10726) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1495::<F>(t10722, t857, t240, t2719);
        let t10744 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1496::<F>(t2735, t2783);
    (t10717, t10719, t10722, t10723, t10726, t10744)
}
