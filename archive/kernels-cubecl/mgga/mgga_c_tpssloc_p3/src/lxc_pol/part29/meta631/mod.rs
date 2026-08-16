//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta631 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2078;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2079;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta631<F: Float>(t16524: F, t23896: F, t45560: F, t7769: F, t16521: F, t6534: F, t1873: F, t55405: F, t23893: F, t12524: F, t26550: F, t16535: F, t7467: F, t26135: F, t3938: F, t12816: F, t191: F, t192: F, t2020: F, t26161: F, t26162: F, t56404: F, t16148: F, t24995: F, t8945: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t86639, t86642, t86646, t86651, t86653, t86655, t86660) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2078::<F>(t16524, t23896, t45560, t7769, t16521, t6534, t1873, t55405, t23893, t12524, t26550, t16535, t7467);
        let (t86668, t86673, t86676, t86679) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2079::<F>(t26135, t3938, t12816, t191, t192, t2020, t26161, t26162, t56404, t16148, t24995, t8945);
    (t86639, t86642, t86646, t86651, t86653, t86655, t86660, t86668, t86673, t86676, t86679)
}
