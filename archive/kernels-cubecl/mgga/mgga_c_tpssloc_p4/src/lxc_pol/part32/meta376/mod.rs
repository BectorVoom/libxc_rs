//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta376 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1432;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1433;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta376<F: Float>(t5151: F, t67: F, t758: F, t12365: F, t1827: F, t12300: F, t12418: F, t820: F, t1351: F, t1799: F, t12289: F, t242: F, t1336: F, t12283: F, t5259: F, t5293: F, t120: F, t5286: F, t5303: F, t1340: F, t16060: F, t3798: F, t5234: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t16171, t16211, t16214, t16224, t16225, t16232) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1432::<F>(t5151, t67, t758, t12365, t1827, t12300, t12418, t820, t1351, t1799, t12289, t242);
        let (t16233, t16239, t16241, t16242, t16269, t16278, t16288) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1433::<F>(t1336, t16232, t12283, t5259, t5293, t120, t5286, t5303, t1340, t16060, t3798, t5234);
    (t16171, t16211, t16214, t16224, t16225, t16233, t16239, t16241, t16242, t16269, t16278, t16288)
}
