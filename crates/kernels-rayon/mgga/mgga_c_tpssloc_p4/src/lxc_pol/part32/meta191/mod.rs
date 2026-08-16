//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta191 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk934;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk935;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk936;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta191(t1268: f64, t1458: f64, t2314: f64, t4026: f64, t4028: f64, t4072: f64, t5113: f64, t671: f64, t1390: f64, t1845: f64, t193: f64, t531: f64, t25: f64, t1799: f64, t571: f64, t3919: f64, t1408: f64, t3664: f64, t2: f64, t514: f64, t584: f64, t606: f64, t1649: f64, t3672: f64, t517: f64, zeta_threshold: f64, t28: f64, t1081: f64, t157: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5118, t5122, t5126) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk934(t1268, t1458, t2314, t4026, t4028, t4072, t5113, t671, t1390, t1845, t193, t531);
        let (t5127, t5131, t5134, t5141, t5142, t5145) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk935(t25, t1799, t571, t3919, t1408, t3664, t2, t514, t584, t606, t1649, t3672, t517, zeta_threshold);
        let t5151 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk936(t28, t1081, t5142, t5145, t584, t157, t5141, zeta_threshold);
    (t5118, t5122, t5126, t5127, t5131, t5134, t5142, t5151)
}
