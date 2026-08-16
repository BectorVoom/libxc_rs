//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta275 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1489;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1490;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta275<F: Float>(t760: F, t9372: F, t37: F, t716: F, t2523: F, t2626: F, t9425: F, t2609: F, t606: F, t706: F, t2475: F, t73: F, t775: F, t853: F, t2710: F, t2793: F, t9285: F, t2470: F, t2804: F, t874: F, t875: F, t9288: F, t2718: F, t860: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t10604, t10605, t10608, t10611, t10612, t10613, t10626) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1489::<F>(t760, t9372, t37, t716, t2523, t2626, t9425, t2609, t606, t706, t2475, t73);
        let (t10631, t10645, t10647, t10651, t10661) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1490::<F>(t775, t853, t2710, t2793, t9285, t2470, t2804, t874, t875, t9288, t2718, t860);
    (t10604, t10605, t10608, t10611, t10612, t10613, t10626, t10631, t10645, t10647, t10651, t10661)
}
