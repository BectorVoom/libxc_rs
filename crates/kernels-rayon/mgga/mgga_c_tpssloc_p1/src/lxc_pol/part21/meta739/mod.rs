//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta739 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2602;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2603;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta739(t10471: f64, t52834: f64, t11737: f64, t11651: f64, t15507: f64, t13969: f64, t15621: f64, t3506: f64, t11791: f64, t5005: f64, t11697: f64, t15477: f64, t3577: f64, t11677: f64, t15027: f64, t3575: f64, t373: f64, t470: f64, t493: f64, t1214: f64, t820: f64, t3624: f64, t52627: f64, t11745: f64, t15503: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t52835, t52836, t52845, t52859, t52872, t52875) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2602(t10471, t52834, t11737, t11651, t15507, t13969, t15621, t3506, t11791, t5005, t11697, t15477, t3577);
        let (t52879, t52893, t52897, t52903, t52906) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2603(t11677, t15027, t3575, t373, t470, t493, t1214, t820, t3624, t52627, t11745, t15503);
    (t52835, t52836, t52845, t52859, t52872, t52875, t52879, t52893, t52897, t52903, t52906)
}
