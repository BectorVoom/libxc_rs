//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 704/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk704(t1608: f64, t1614: f64, t3070: f64, t3066: f64, t7839: f64, t3037: f64, t3029: f64, t7847: f64, t7858: f64, t7906: f64, t8051: f64, t938: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11094 = t1608 * t3070 * t1614;
    let t11095 = t3066 * t7839;
    let t11098 = t3037 * t7839;
    let t11104 = t3029 * t7847;
    let t11109 = t7906 * t7858;
    let t11115 = t8051 * t938;
    (t11094, t11095, t11098, t11104, t11109, t11115)
}
