//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta161 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk869;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk870;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk871;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta161<F: Float>(t354: F, t471: F, t1214: F, t3766: F, t487: F, t460: F, t3302: F, t3603: F, t1248: F, t3781: F, t670: F, t93: F, t198: F, t530: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t5458, t5462) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk869::<F>(t354, t471, t1214, t3766, t487);
        let (t5463, t5464, t5465, t5477) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk870::<F>(t460, t5462, t3302, t3603, t1248, t3781, t487);
        let (t5478, t5480, t5523, t5536) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk871::<F>(t460, t5477, t1248, t3302, t471, t670, t93, t198, t530);
    (t5458, t5462, t5463, t5464, t5465, t5477, t5478, t5480, t5523, t5536)
}
