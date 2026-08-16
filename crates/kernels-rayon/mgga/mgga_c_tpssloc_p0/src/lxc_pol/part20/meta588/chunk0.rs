//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2166/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2166(t2296: f64, t3241: f64, t11778: f64, t154: f64, t1091: f64, t9698: f64) -> (f64, f64, f64) {
    let t43791 = 1.0_f64 / t3241 / t2296;
    let t43809 = t154 * t11778;
    let t43816 = t9698 * t1091;
    (t43791, t43809, t43816)
}
