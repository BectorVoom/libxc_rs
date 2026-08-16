//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta446 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1781;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1782;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1783;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta446(t252: f64, t2631: f64, t2632: f64, t22996: f64, t1888: f64, t6579: f64, t6649: f64, t232: f64, t6646: f64, t1902: f64, t2627: f64, t2633: f64, t1879: f64, t22715: f64, t1906: f64, t2679: f64, t6657: f64, t1894: f64, t2710: f64, t214: f64, t1880: f64, t1909: f64, t22984: f64, t22990: f64, t22993: f64, t2613: f64, t2617: f64, t6658: f64, t6660: f64, t808: f64, t812: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22998, t22999, t23000, t23002, t23003, t23004, t23005, t23006, t23009) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1781(t252, t2631, t2632, t22996, t1888, t6579, t6649, t232, t6646, t1902, t2627, t2633);
        let t23012 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1782(t1879, t22715);
        let (t23014, t23016, t23020, t23021, t23024) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1783(t1906, t23012, t2679, t6657, t1894, t2710, t214, t1880, t1909, t22984, t22990, t22993, t23000, t23003, t23006, t23009, t2613, t2617, t6658, t6660, t808, t812);
    (t22998, t22999, t23002, t23004, t23005, t23009, t23012, t23014, t23016, t23020, t23021, t23024)
}
