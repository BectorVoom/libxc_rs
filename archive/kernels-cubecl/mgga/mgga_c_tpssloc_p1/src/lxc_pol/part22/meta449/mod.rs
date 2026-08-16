//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta449 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1806;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1807;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta449<F: Float>(t3866: F, t6427: F, t6431: F, t19735: F, t5248: F, t5249: F, t16242: F, t3805: F, t6394: F, t120: F, t6414: F, t3807: F, t1352: F, t5250: F, t5287: F, t19871: F, t6330: F, t12419: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t19940, t19942, t19945, t19951, t19956) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1806::<F>(t3866, t6427, t6431, t19735, t5248, t5249, t16242, t3805, t6394, t120, t6414);
        let (t19958, t19962, t19966, t19972, t19976, t19981) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1807::<F>(t19956, t3805, t3807, t1352, t5248, t5250, t5249, t5287, t19871, t120, t6330, t12419);
    (t19940, t19942, t19945, t19951, t19956, t19958, t19962, t19966, t19972, t19976, t19981)
}
