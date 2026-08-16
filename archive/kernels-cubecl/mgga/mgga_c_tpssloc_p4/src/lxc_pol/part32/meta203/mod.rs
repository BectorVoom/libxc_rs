//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta203 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk983;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk984;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk985;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk986;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta203<F: Float>(t33: F, t5427: F, t2291: F, t5392: F, t5398: F, t634: F, t2298: F, t638: F, t72: F, t1411: F, t1427: F, t1434: F, t5393: F, t5400: F, t5403: F, t66: F, t80: F, t5: F, t1437: F, t2240: F, t3953: F, t5385: F, t5389: F, t605: F, t86: F, t112: F, t1458: F, t89: F, t1774: F, t1453: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t5428, t5441, t5442, t5445) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk983::<F>(t33, t5427, t2291, t5392, t5398, t634, t2298, t638, t72, t1411, t1427, t1434, t5393, t5400, t5403, t66, t80);
        let (t5449, t5450) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk984::<F>(t5, t1437, t2240, t3953, t5385, t5389, t5445, t605, t86, t112);
        let t5456 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk985::<F>(t1458);
        let (t5457, t5460, t5464) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk986::<F>(t5456, t89, t1458, t1774, t1453);
    (t5428, t5441, t5442, t5445, t5449, t5450, t5456, t5457, t5460, t5464)
}
