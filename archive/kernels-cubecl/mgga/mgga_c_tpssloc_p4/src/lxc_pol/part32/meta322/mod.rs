//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta322 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1351;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1352;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1353;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta322<F: Float>(t3585: F, t820: F, t10401: F, t3575: F, t3610: F, t3624: F, t3521: F, t1190: F, t3030: F, t3032: F, t3505: F, t10469: F, t466: F, t10471: F, t1208: F, t478: F, t10477: F, t483: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t11668, t11678, t11692, t11697, t11707, t11708, t11709, t11712) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1351::<F>(t3585, t820, t10401, t3575, t3610, t3624, t3521, t1190, t3030, t3032, t3505, t10469, t466);
        let t11713 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1352::<F>(t10471, t11712);
        let (t11715, t11716, t11717) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1353::<F>(t1208, t478, t10477, t483);
    (t11668, t11678, t11692, t11697, t11707, t11708, t11709, t11712, t11713, t11715, t11716, t11717)
}
