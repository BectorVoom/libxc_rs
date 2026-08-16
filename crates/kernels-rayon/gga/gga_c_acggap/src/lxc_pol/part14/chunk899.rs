//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 899/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk899(t1170: f64, t1171: f64, t30538: f64, t1108: f64, t7746: f64, t1113: f64, t14046: f64, t7336: f64, t7643: f64, t1973: f64, t7630: f64, t1985: f64, t30231: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t30948 = t1170 * t30538 * t1171;
    let t30978 = t7746 * t1108;
    let t30982 = t7746 * t1113;
    let t30984 = t14046 * t7336;
    let t30985 = t30984 * t7643;
    let t30987 = t7630 * t1973;
    let t30989 = t30231 * t1985;
    (t30948, t30978, t30982, t30984, t30985, t30987, t30989)
}
