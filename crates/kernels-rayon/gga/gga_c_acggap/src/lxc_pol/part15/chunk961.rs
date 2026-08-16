//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 961/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk961(t7839: f64, t8518: f64, t8522: f64, t31699: f64, t8526: f64, t7637: f64, t8506: f64, t368: f64, t4806: f64, t1980: f64, t7476: f64, t2304: f64, t7780: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t34035 = t7839 * t8518;
    let t34037 = t7839 * t8522;
    let t34039 = t31699 * t8526;
    let t34043 = t7637 * t8506;
    let t34050 = t368 * t4806;
    let t34052 = t1980 * t7476 * t34050;
    let t34054 = t7780 * t2304;
    (t34035, t34037, t34039, t34043, t34050, t34052, t34054)
}
