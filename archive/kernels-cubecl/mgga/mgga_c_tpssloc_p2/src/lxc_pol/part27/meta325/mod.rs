//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta325 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1401;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1402;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta325<F: Float>(t10469: F, t466: F, t10471: F, t1208: F, t478: F, t10477: F, t483: F, t3508: F, t475: F, t3503: F, t11708: F, t3514: F, t1210: F, t248: F, t3509: F, t3570: F, t3506: F, t135: F, t3561: F, t1174: F, t3247: F, t415: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t11712, t11713, t11715, t11717, t11719, t11721, t11728, t11734) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1401::<F>(t10469, t466, t10471, t1208, t478, t10477, t483, t3508, t475, t3503, t11708, t3514);
        let (t11738, t11746, t11755, t11778) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1402::<F>(t11717, t1210, t11713, t248, t3509, t3570, t3506, t135, t3561, t1174, t3247, t415);
    (t11712, t11715, t11719, t11721, t11728, t11734, t11738, t11746, t11755, t11778)
}
