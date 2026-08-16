//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta342 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1735;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1736;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta342(t12890: f64, t185: f64, t2250: f64, t4195: f64, t4194: f64, t4303: f64, t870: f64, t262: f64, t4119: f64, t2553: f64, t4315: f64, t9717: f64, t12850: f64, t12854: f64, t12860: f64, t12861: f64, t12889: f64, t1877: f64, t2522: f64, t4310: f64, t4314: f64, t776: f64, t868: f64, t9457: f64, t9462: f64, t9469: f64, t9476: f64, t9484: f64, t9496: f64, t9715: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t12891, t12892, t12894, t12895, t12899, t12903, t12906) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1735(t12890, t185, t2250, t4195, t4194, t4303, t870, t262, t4119, t2553, t4315, t9717);
        let t12907 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1736(t12850, t12854, t12860, t12861, t12889, t12891, t12894, t12895, t12899, t12903, t12906, t1877, t2522, t2553, t4310, t4314, t776, t868, t9457, t9462, t9469, t9476, t9484, t9496, t9715);
    (t12891, t12892, t12894, t12895, t12906, t12907)
}
