//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta563 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1972;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1973;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta563<F: Float>(t460: F, t491: F, t7286: F, t27453: F, t27721: F, t466: F, t7280: F, t7999: F, t1186: F, t8010: F, t1170: F, t2121: F, t8034: F, t7287: F, t24567: F, t8014: F, t225: F, t8018: F, t1252: F, t15797: F, t2155: F, t24589: F, t24891: F, t3487: F, t4945: F, t498: F, t5055: F, t5089: F, t7283: F, t7296: F, t7351: F, t7356: F, t7392: F, t8088: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t27798, t27799, t27800, t27805, t27808, t27812, t27818) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1972::<F>(t460, t491, t7286, t27453, t27721, t466, t7280, t7999, t1186, t8010, t1170, t2121);
        let (t27820, t27821, t27826, t27830, t27832) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1973::<F>(t491, t8034, t7287, t24567, t8014, t225, t8018, t1252, t15797, t2155, t24589, t24891, t27800, t27805, t27808, t27812, t27818, t3487, t4945, t498, t5055, t5089, t7283, t7296, t7351, t7356, t7392, t7999, t8088);
    (t27798, t27799, t27800, t27805, t27812, t27820, t27821, t27826, t27830, t27832)
}
