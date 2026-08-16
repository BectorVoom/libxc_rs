//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 910/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk910(t30904: f64, t1035: f64, t1039: f64, t7613: f64, t1200: f64, t7605: f64, t1988: f64, t7535: f64, t30589: f64, t7548: f64, t2109: f64, t7630: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30905 = 0.25724410870841842183e-2_f64 * t30904;
    let t30907 = t1035 * t7613 * t1039;
    let t30908 = 0.12004725073059526352e-1_f64 * t30907;
    let t30916 = t7605 * t1200;
    let t30918 = t1988 * t7535;
    let t30920 = t30589 * t7548;
    let t30921 = 0.41930789719472202756e-2_f64 * t30920;
    let t30924 = t7630 * t2109;
    (t30905, t30908, t30916, t30918, t30921, t30924)
}
