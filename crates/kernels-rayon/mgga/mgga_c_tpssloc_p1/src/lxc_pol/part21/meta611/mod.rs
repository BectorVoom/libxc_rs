//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta611 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2377;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2378;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2379;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta611(t12088: f64, t2528: f64, t3691: f64, t9919: f64, t2367: f64, t2508: f64, t39378: f64, t9493: f64, t1294: f64, t9713: f64, t2405: f64, t2412: f64, t9479: f64, t9481: f64, t39273: f64, t39275: f64, t39278: f64, t39281: f64, t39284: f64, t39289: f64, t39291: f64, t39293: f64, t39295: f64, t39298: f64, t683: f64, t702: f64, t746: f64, t9720: f64, t1285: f64, t9214: f64, t39253: f64, t9453: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t39530, t39532, t39535, t39537, t39539, t39540, t39549) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2377(t12088, t2528, t3691, t9919, t2367, t2508, t39378, t9493, t1294, t9713, t2405, t2412, t9479, t9481);
        let t39563 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2378(t39273, t39275, t39278, t39281, t39284, t39289, t39291, t39293, t39295, t39298, t683, t702);
        let (t39568, t39570, t39571, t39585) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2379(t39378, t746, t9720, t1294, t1285, t9214, t39253, t702, t9453);
    (t39530, t39532, t39535, t39537, t39539, t39540, t39549, t39563, t39568, t39570, t39571, t39585)
}
