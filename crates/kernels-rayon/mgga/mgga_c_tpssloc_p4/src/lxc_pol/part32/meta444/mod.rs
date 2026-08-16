//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta444 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1699;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1700;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1701;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1702;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta444(t225: f64, t6956: f64, t562: f64, t794: f64, t6907: f64, t6897: f64, t131: f64, t557: f64, t209: f64, t1878: f64, t212: f64, t6968: f64, t22642: f64, t268: f64, t534: f64, t6559: f64, t1338: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22670, t22674) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1699(t225, t6956, t562, t794);
        let (t22675, t22676, t22683, t22684, t22685, t22690) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1700(t22674, t6907, t6897, t131, t557, t209, t1878, t212, t225);
        let (t22691, t22693, t22704) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1701(t22690, t6968, t22642, t268, t534, t6559);
        let t22705 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1702(t1338, t22690);
    (t22670, t22674, t22675, t22676, t22683, t22684, t22685, t22690, t22691, t22693, t22704, t22705)
}
