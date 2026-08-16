//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta256 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1381;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1382;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta256(t11135: f64, t11203: f64, t1124: f64, t3356: f64, t3355: f64, t432: f64, t427: f64, t1094: f64, t3263: f64, t11153: f64, t461: f64, t1176: f64, t698: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11369, t11372, t11415, t11419, t11420) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1381(t11135, t11203, t1124, t3356, t3355, t432, t427);
        let (t11424, t11444, t11459, t11487, t11516, t11529) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1382(t1094, t3263, t11135, t11203, t11153, t461, t1176, t698);
    (t11369, t11372, t11415, t11419, t11420, t11424, t11444, t11459, t11487, t11516, t11529)
}
