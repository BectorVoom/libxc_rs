//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta586 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2320;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2321;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2322;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta586(t3726: f64, t6358: f64, t213: f64, t6347: f64, t1307: f64, t221: f64, t12228: f64, t12236: f64, t16078: f64, t16083: f64, t16099: f64, t16106: f64, t16108: f64, t16113: f64, t16119: f64, t5195: f64, t19790: f64, t225: f64, t1814: f64, t5343: f64, t3901: f64, t6420: f64, t6378: f64, t68: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t19793, t19795, t19803) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2320(t3726, t6358, t213, t6347, t1307, t221, t12228, t12236, t16078, t16083, t16099, t16106, t16108, t16113, t16119, t5195);
        let (t19804, t19805, t19810) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2321(t19790, t19803, t225, t1814, t5343);
        let (t19813, t19815) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2322(t3901, t6420, t6378, t68);
    (t19793, t19795, t19804, t19805, t19810, t19813, t19815)
}
