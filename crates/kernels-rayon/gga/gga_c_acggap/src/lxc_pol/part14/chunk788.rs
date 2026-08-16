//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 788/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk788(t1588: f64, t2001: f64, t1988: f64, t2327: f64, t1487: f64, t6: f64, t422: f64, t599: f64, t598: f64, t1982: f64, t1983: f64, t507: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8849 = t2001 * t1588;
    let t8851 = t1988 * t2327;
    let t8853 = t6 * t1487;
    let t8855 = t422 * t8853 * t599;
    let t8856 = t598 * t8855;
    let t8859 = t1982 * t507 * t1983;
    (t8849, t8851, t8853, t8855, t8856, t8859)
}
