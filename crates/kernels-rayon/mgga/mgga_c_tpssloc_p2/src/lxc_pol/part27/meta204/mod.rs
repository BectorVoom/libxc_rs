//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta204 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1019;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1020;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1021;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1022;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta204(t381: f64, t4552: f64, t1049: f64, t1603: f64, t1604: f64, t225: f64, t1625: f64, t990: f64, t4343: f64, t977: f64, t2979: f64, t4338: f64, t1539: f64, t248: f64, t3051: f64, t1041: f64, t1616: f64, t884: f64, t3071: f64, t1023: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4553, t4555, t4557) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1019(t381, t4552, t1049, t1603, t1604, t225);
        let (t4559, t4562, t4565, t4571) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1020(t1625, t990, t4343, t977, t2979, t4338, t1539, t248, t3051);
        let (t4572, t4574, t4575) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1021(t1041, t4571, t1616, t884, t3071);
        let (t4578, t4579) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1022(t1023, t1539, t3071);
    (t4553, t4555, t4557, t4559, t4562, t4565, t4571, t4572, t4574, t4575, t4578, t4579)
}
