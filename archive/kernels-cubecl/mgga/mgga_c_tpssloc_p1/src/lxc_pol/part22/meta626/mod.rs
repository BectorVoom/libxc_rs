//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta626 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2160;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2161;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta626<F: Float>(t16060: F, t3865: F, t1831: F, t40292: F, t12345: F, t5314: F, t40018: F, t5223: F, t12282: F, t5234: F, t12189: F, t5227: F, t40281: F, t5303: F, t5247: F, t820: F, t12250: F, t1824: F, t3789: F, t12384: F, t5293: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t53906, t53918, t53920, t53928, t53945, t53984) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2160::<F>(t16060, t3865, t1831, t40292, t12345, t5314, t40018, t5223, t12282, t5234, t12189, t5227);
        let (t53985, t53998, t54013, t54014, t54023, t54042, t54047) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2161::<F>(t53984, t40281, t5303, t5247, t820, t12250, t1824, t16060, t3789, t12384, t5234, t5293);
    (t53906, t53918, t53920, t53928, t53945, t53985, t53998, t54013, t54014, t54023, t54042, t54047)
}
