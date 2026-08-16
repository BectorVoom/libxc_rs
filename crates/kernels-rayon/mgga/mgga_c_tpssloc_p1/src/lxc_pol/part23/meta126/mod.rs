//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta126 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk624;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk625;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk626;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk627;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta126(t3787: f64, t68: f64, t544: f64, t1824: f64, t562: f64, t1338: f64, t1834: f64, t112: f64, t1851: f64, t2218: f64, t2220: f64, t2222: f64, t2224: f64, t2226: f64, t2228: f64, t2232: f64, t1437: f64, t1409: f64, t65: f64, t11: f64, t2219: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5333, t5334, t5335, t5343, t5344, t5348, t5371, t5385) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk624(t3787, t68, t544, t1824, t562, t1338, t1834, t112, t1851, t2218, t2220, t2222, t2224, t2226, t2228, t2232);
        let t5389 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk625(t1437);
        let t5392 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk626(t1409);
        let (t5393, t5396) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk627(t5392, t65, t11, t2219);
    (t5333, t5334, t5335, t5343, t5344, t5348, t5371, t5385, t5389, t5392, t5393, t5396)
}
