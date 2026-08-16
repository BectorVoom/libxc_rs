//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1053/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1053(t162: f64, t22858: f64, t6792: f64, t6799: f64, t1948: f64, t6785: f64, t2034: f64, t2037: f64, t6893: f64, t127: f64, t616: f64, t6877: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22859 = t162 * t22858;
    let t22862 = t6799 * t6792;
    let t22864 = t6785 * t1948;
    let t22865 = t2034 * t22864;
    let t22868 = t6893 * t2037;
    let t22871 = t6877 * t127 * t616;
    (t22859, t22862, t22864, t22865, t22868, t22871)
}
