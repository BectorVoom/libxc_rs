//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1132/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1132(t1181: f64, t5969: f64, t599: f64, t7493: f64, t1839: f64, t1983: f64, t7585: f64, t7586: f64, t7839: f64, t9641: f64, t1165: f64, t2068: f64, t604: f64, t6069: f64) -> (f64, f64, f64, f64) {
    let t39669 = t7493 * t1181 * t599 * t5969;
    let t39673 = t7585 * t7586 * t1983 * t1839;
    let t39675 = t7839 * t9641;
    let t39679 = t2068 * t1165 * t604 * t6069;
    (t39669, t39673, t39675, t39679)
}
