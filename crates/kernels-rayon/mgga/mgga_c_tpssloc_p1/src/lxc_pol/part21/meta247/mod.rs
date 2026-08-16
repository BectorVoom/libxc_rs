//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta247 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1448;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1449;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1450;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta247(t457: f64, t6138: f64, t460: f64, t974: f64, t1714: f64, t1174: f64, t1710: f64, t1717: f64, t3430: f64, t3447: f64, t463: f64, t4887: f64, t4889: f64, t4897: f64, t4917: f64, t6109: f64, t6120: f64, t6123: f64, t6127: f64, t6131: f64, t491: f64, t1720: f64, t1751: f64, t1730: f64, t1743: f64, t1417: f64, t47: f64, t480: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6140, t6141, t6144) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1448(t457, t6138, t460, t974, t1714);
        let (t6146, t6150) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1449(t457, t6144, t460, t974, t1174, t1710, t1717, t3430, t3447, t463, t4887, t4889, t4897, t4917, t6109, t6120, t6123, t6127, t6131, t6141);
        let (t6151, t6153, t6158, t6163) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1450(t491, t6150, t1720, t1751, t1730, t1743, t1417, t47, t480);
    (t6140, t6144, t6146, t6150, t6151, t6153, t6158, t6163)
}
