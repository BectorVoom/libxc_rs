//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta299 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1212;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1213;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta299<F: Float>(t1041: F, t10459: F, t1008: F, t349: F, t1011: F, t1013: F, t363: F, t3034: F, t6793: F, t368: F, t3131: F, t360: F, t248: F, t2776: F, t3051: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t10460, t10469, t10470, t10471) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1212::<F>(t1041, t10459, t1008, t349, t1011);
        let (t10472, t10474, t10477, t10478, t10480, t10482, t10489) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1213::<F>(t10470, t10471, t1013, t363, t3034, t6793, t368, t3131, t360, t248, t2776, t3051);
    (t10460, t10469, t10470, t10471, t10472, t10474, t10477, t10478, t10480, t10482, t10489)
}
