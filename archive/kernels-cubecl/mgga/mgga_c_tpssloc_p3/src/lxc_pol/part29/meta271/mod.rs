//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta271 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1268;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1269;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta271<F: Float>(t1530: F, t28: F, t1649: F, t1877: F, t1915: F, t2522: F, t6670: F, t7541: F, t7650: F, t1873: F, t4028: F, t1458: F, t88: F, t1268: F, t7467: F, t1778: F, t191: F, t192: F) -> (F, F, F, F, F, F, F, F) {
        let (t7656, t7663, t7675, t7676) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1268::<F>(t1530, t28, t1649, t1877, t1915, t2522, t6670, t7541, t7650, t1873, t4028, t1458, t88);
        let (t7678, t7680, t7684, t7685) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1269::<F>(t1873, t7676, t1268, t7467, t1778, t191, t192);
    (t7656, t7663, t7675, t7676, t7678, t7680, t7684, t7685)
}
