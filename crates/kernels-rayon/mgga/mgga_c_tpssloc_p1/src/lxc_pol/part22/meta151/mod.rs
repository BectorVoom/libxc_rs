//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta151 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk955;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk956;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk957;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta151(t1625: f64, t990: f64, t4343: f64, t977: f64, t2979: f64, t4338: f64, t1539: f64, t248: f64, t3051: f64, t1041: f64, t1616: f64, t884: f64, t3071: f64, t1023: f64, t247: f64, t375: f64, t1043: f64, t2775: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4559, t4562, t4565, t4571) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk955(t1625, t990, t4343, t977, t2979, t4338, t1539, t248, t3051);
        let (t4572, t4574, t4575, t4578, t4579, t4582) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk956(t1041, t4571, t1616, t884, t3071, t1023, t1539, t247, t375);
        let t4583 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk957(t1043, t2775);
    (t4559, t4562, t4565, t4571, t4572, t4574, t4575, t4578, t4579, t4582, t4583)
}
