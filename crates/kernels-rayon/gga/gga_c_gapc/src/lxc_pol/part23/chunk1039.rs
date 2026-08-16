//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 1039/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk1039(t1875: f64, t2972: f64, t134: f64, t8957: f64, t5549: f64, t116: f64, t126: f64, t1038: f64, t1602: f64, t1908: f64, t19509: f64, t681: f64) -> (f64, f64, f64, f64, f64) {
    let t26887 = t1875 * t2972;
    let t26995 = t8957 * t134;
    let t26996 = t26995 * t5549;
    let t27036 = t116 * t126;
    let t27043 = t1908 * t681 * t1038 * t19509 * t1602;
    (t26887, t26995, t26996, t27036, t27043)
}
