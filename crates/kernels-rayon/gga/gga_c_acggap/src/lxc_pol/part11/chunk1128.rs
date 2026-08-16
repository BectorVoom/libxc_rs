//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1128/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1128(t1983: f64, t7585: f64, t7586: f64, t8402: f64, t30105: f64, t8897: f64, t1181: f64, t2068: f64, t33976: f64, t599: f64, t20433: f64, t604: f64) -> (f64, f64, f64, f64) {
    let t35484 = t7585 * t7586 * t1983 * t8402;
    let t35485 = 0.14291339372689912324e-3_f64 * t35484;
    let t35486 = t30105 * t8897;
    let t35490 = t2068 * t1181 * t599 * t33976;
    let t35494 = t2068 * t1181 * t604 * t20433;
    (t35485, t35486, t35490, t35494)
}
