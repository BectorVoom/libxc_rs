//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta918 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3127;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3128;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta918<F: Float>(t12046: F, t1647: F, t16551: F, t989: F, t12153: F, t4746: F, t16237: F, t359: F, t15654: F, t3286: F, t16543: F, t3046: F, t4995: F, t15669: F, t1651: F, t378: F, t342: F, t43400: F, t1086: F, t15886: F, t3057: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t55599, t55632, t55646, t55649, t55685, t55701) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3127::<F>(t12046, t1647, t16551, t989, t12153, t4746, t16237, t359, t15654, t3286, t16543, t3046);
        let (t55732, t55747, t55764, t55805, t55868, t55887) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3128::<F>(t4746, t4995, t15669, t3286, t1651, t378, t342, t43400, t1086, t15886, t16543, t3057);
    (t55599, t55632, t55646, t55649, t55685, t55701, t55732, t55747, t55764, t55805, t55868, t55887)
}
