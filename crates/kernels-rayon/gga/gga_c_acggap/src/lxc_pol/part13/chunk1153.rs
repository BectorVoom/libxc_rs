//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1153/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1153(t31603: f64, t1588: f64, t7614: f64, t1988: f64, t8855: f64, t7799: f64, t8859: f64, t422: f64, t4875: f64, t598: f64, t599: f64, t6: f64) -> (f64, f64, f64, f64, f64) {
    let t35812 = 13.0_f64 / 144.0_f64 * t31603;
    let t35814 = t7614 * t1588;
    let t35816 = t1988 * t8855;
    let t35817 = 0.21437009059034868486e-3_f64 * t35816;
    let t35818 = t7799 * t8859;
    let t35823 = t598 * t422 * t6 * t4875 * t599;
    (t35812, t35814, t35817, t35818, t35823)
}
