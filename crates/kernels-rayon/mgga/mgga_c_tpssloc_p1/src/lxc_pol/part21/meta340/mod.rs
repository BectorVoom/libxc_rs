//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta340 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1730;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1731;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1732;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta340(t1268: f64, t12724: f64, t12725: f64, t12728: f64, t12734: f64, t12739: f64, t12813: f64, t1458: f64, t2314: f64, t2363: f64, t4028: f64, t4072: f64, t5113: f64, t671: f64, t9348: f64, t89: f64, t12545: f64, t12550: f64, t12557: f64, t1442: f64, t1459: f64, t1849: f64, t2323: f64, t2364: f64, t3652: f64, t3660: f64, t4034: f64, t4037: f64, t4073: f64, t574: f64, t652: f64, t672: f64, t510: f64, t4098: f64, t751: f64, t2752: f64, t4303: f64, t172: f64, t4095: f64, t763: f64, t1472: f64, t2517: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t12816 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1730(t1268, t12724, t12725, t12728, t12734, t12739, t12813, t1458, t2314, t2363, t4028, t4072, t5113, t671, t9348);
        let (t12823, t12832) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1731(t2363, t89, t12545, t12550, t12557, t12725, t12734, t12816, t1442, t1459, t1849, t2314, t2323, t2364, t3652, t3660, t4028, t4034, t4037, t4073, t574, t652, t672, t9348);
        let (t12835, t12841, t12850, t12854, t12858, t12860, t12861) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1732(t12813, t510, t1458, t3652, t4098, t751, t2752, t4303, t172, t4095, t763, t1472, t2517);
    (t12816, t12823, t12832, t12835, t12841, t12850, t12854, t12858, t12860, t12861)
}
