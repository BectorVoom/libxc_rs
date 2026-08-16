//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta191 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk934;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk935;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk936;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta191<F: Float>(t1268: F, t1458: F, t2314: F, t4026: F, t4028: F, t4072: F, t5113: F, t671: F, t1390: F, t1845: F, t193: F, t531: F, t25: F, t1799: F, t571: F, t3919: F, t1408: F, t3664: F, t2: F, t514: F, t584: F, t606: F, t1649: F, t3672: F, t517: F, zeta_threshold: F, t28: F, t1081: F, t157: F) -> (F, F, F, F, F, F, F, F) {
        let (t5118, t5122, t5126) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk934::<F>(t1268, t1458, t2314, t4026, t4028, t4072, t5113, t671, t1390, t1845, t193, t531);
        let (t5127, t5131, t5134, t5141, t5142, t5145) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk935::<F>(t25, t1799, t571, t3919, t1408, t3664, t2, t514, t584, t606, t1649, t3672, t517, zeta_threshold);
        let t5151 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk936::<F>(t28, t1081, t5142, t5145, t584, t157, t5141, zeta_threshold);
    (t5118, t5122, t5126, t5127, t5131, t5134, t5142, t5151)
}
