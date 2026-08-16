//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta291 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1447;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1448;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1449;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta291(t2770: f64, t3966: f64, t10216: f64, t1409: f64, t2775: f64, t4389: f64, t699: f64, t4386: f64, t10277: f64, t4339: f64, t690: f64, t4344: f64, t1540: f64, t2394: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13527, t13536, t13541, t13550, t13551, t13552, t13554, t13563) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1447(t2770, t3966, t10216, t1409, t2775, t4389, t699, t4386, t10277, t4339, t690);
        let t13566 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1448(t4344, t690);
        let (t13567, t13598) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1449(t13566, t1540, t2394);
    (t13527, t13536, t13541, t13550, t13551, t13552, t13554, t13563, t13566, t13567, t13598)
}
