//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta303 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1365;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1366;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta303<F: Float>(t1013: F, t363: F, t3034: F, t6793: F, t368: F, t10472: F, t3131: F, t360: F, t248: F, t2776: F, t3051: F, t1041: F, t3103: F, t3109: F, t3114: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t10474, t10475, t10477, t10478) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1365::<F>(t1013, t363, t3034, t6793, t368);
        let (t10480, t10482, t10489, t10490, t10496, t10504) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1366::<F>(t10475, t10478, t10472, t3131, t360, t248, t2776, t3051, t1041, t3103, t3109, t3114);
    (t10474, t10475, t10477, t10478, t10480, t10482, t10489, t10490, t10496, t10504)
}
