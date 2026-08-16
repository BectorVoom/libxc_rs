//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 746/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk746(t11175: f64, t17: f64, t9: f64, t3141: f64, t8282: f64, t959: f64, t2: f64, t3103: f64, t1587: f64, t432: f64, t1588: f64, t3149: f64, t7750: f64) -> (f64, f64, f64, f64, f64) {
    let t11717 = t9 * t11175 * t17;
    let t11718 = t11717 * t3141;
    let t11720 = t8282 * t959;
    let t11722 = t2 * t3103;
    let t11724 = t1587 * t11722 * t432;
    let t11728 = t7750 * t3149 * t1588;
    (t11717, t11718, t11720, t11724, t11728)
}
