//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 706/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk706(t3057: f64, t6: f64, t39: f64, t930: f64, t3056: f64, t77: f64, t3020: f64, t122: f64, t938: f64, t1593: f64, t1595: f64, t35: f64) -> (f64, f64, f64, f64, f64) {
    let t11127 = t3057 * t6;
    let t11131 = t930 * t39;
    let t11135 = t77 * t3056;
    let t11136 = t3020 * t11135;
    let t11139 = t938 * t122;
    let t11140 = t1593 * t1595;
    let t11141 = t11140 * t35;
    (t11127, t11131, t11136, t11139, t11141)
}
