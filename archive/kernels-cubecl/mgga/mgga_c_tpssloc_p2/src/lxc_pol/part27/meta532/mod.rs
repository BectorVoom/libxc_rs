//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta532 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1947;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta532<F: Float>(t1458: F, t6862: F, t4028: F, t6535: F, t19577: F, t8643: F, t22574: F, t7458: F, t2314: F, t7461: F, t4034: F, t1873: F, t5107: F) -> (F, F, F, F, F, F, F, F) {
        let (t25965, t25969, t25971, t25973, t25975, t25977, t25979, t25980) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1947::<F>(t1458, t6862, t4028, t6535, t19577, t8643, t22574, t7458, t2314, t7461, t4034, t1873, t5107);
    (t25965, t25969, t25971, t25973, t25975, t25977, t25979, t25980)
}
