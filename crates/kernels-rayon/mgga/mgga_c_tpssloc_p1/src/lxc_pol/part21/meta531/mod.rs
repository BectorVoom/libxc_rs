//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta531 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2189;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2190;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta531(t5914: f64, t990: f64, t17875: f64, t381: f64, t1049: f64, t5848: f64, t1065: f64, t5943: f64, t3174: f64, t1625: f64, t4552: f64, t5919: f64, t10165: f64, t225: f64, t5915: f64, t5872: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18053, t18057, t18059, t18062, t18065, t18070) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2189(t5914, t990, t17875, t381, t1049, t5848, t1065, t5943, t3174, t1625, t4552, t5919);
        let (t18071, t18074, t18080) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2190(t10165, t18070, t225, t5915, t1049, t5872);
    (t18053, t18057, t18059, t18062, t18065, t18071, t18074, t18080)
}
