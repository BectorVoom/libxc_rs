//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta203 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk983;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk984;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk985;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk986;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta203(t33: f64, t5427: f64, t2291: f64, t5392: f64, t5398: f64, t634: f64, t2298: f64, t638: f64, t72: f64, t1411: f64, t1427: f64, t1434: f64, t5393: f64, t5400: f64, t5403: f64, t66: f64, t80: f64, t5: f64, t1437: f64, t2240: f64, t3953: f64, t5385: f64, t5389: f64, t605: f64, t86: f64, t112: f64, t1458: f64, t89: f64, t1774: f64, t1453: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5428, t5441, t5442, t5445) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk983(t33, t5427, t2291, t5392, t5398, t634, t2298, t638, t72, t1411, t1427, t1434, t5393, t5400, t5403, t66, t80);
        let (t5449, t5450) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk984(t5, t1437, t2240, t3953, t5385, t5389, t5445, t605, t86, t112);
        let t5456 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk985(t1458);
        let (t5457, t5460, t5464) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk986(t5456, t89, t1458, t1774, t1453);
    (t5428, t5441, t5442, t5445, t5449, t5450, t5456, t5457, t5460, t5464)
}
