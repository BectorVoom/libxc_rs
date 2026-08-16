//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta655 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2453;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2454;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta655(t11045: f64, t42332: f64, t42340: f64, t42341: f64, t43288: f64, t23508: f64, t43292: f64, t10163: f64, t386: f64, t68: f64, t3215: f64, t3399: f64, t3402: f64, t3639: f64, t2394: f64, t3244: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t43562, t43576, t43577, t43604, t43637, t43688) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2453(t11045, t42332, t42340, t42341, t43288, t23508, t43292, t10163, t386, t68, t3215, t3399);
        let (t43689, t43692, t43706, t43748) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2454(t43688, t3402, t3639, t2394, t3244);
    (t43562, t43576, t43577, t43604, t43637, t43689, t43692, t43706, t43748)
}
