//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta252 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1368;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1369;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1370;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta252(t360: f64, t6739: f64, t10471: f64, t10474: f64, t10470: f64, t10482: f64, t3127: f64, t3131: f64, t3215: f64, t390: f64, t268: f64, t405: f64, t6546: f64, t1091: f64, t2394: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11048, t11058, t11059) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1368(t360, t6739, t10471, t10474, t10470);
        let (t11060, t11064, t11065) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1369(t10482, t6739, t10471, t3127, t10470);
        let (t11066, t11094, t11135, t11136, t11137) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1370(t3131, t6739, t3215, t390, t268, t405, t6546, t1091, t2394);
    (t11048, t11058, t11059, t11060, t11064, t11065, t11066, t11094, t11135, t11136, t11137)
}
