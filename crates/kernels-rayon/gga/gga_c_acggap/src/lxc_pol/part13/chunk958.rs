//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 958/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk958(t2016: f64, t7592: f64, t3378: f64, t7560: f64, t1177: f64, t13364: f64, t31115: f64, t31116: f64, t30049: f64, t7461: f64, t1089: f64, t1198: f64, t2079: f64, t2080: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31822 = t2016 * t7592;
    let t31824 = t3378 * t7560;
    let t31825 = t31824 * t1177;
    let t31832 = t31115 * t13364 * t31116;
    let t31839 = t30049 * t7461;
    let t31840 = 0.42874018118069736972e-3_f64 * t31839;
    let t31843 = t2079 * t1089 * t1198 * t2080;
    (t31822, t31824, t31825, t31832, t31840, t31843)
}
