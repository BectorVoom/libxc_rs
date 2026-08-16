//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 667/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk667(t442: f64, t505: f64, t1037: f64, t1431: f64, t1036: f64, t1386: f64, t515: f64, t1552: f64, t200: f64, t172: f64, t21: f64, t5: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4925 = t442 * t505;
    let t4939 = t1037 * t1431;
    let t4940 = t1036 * t4939;
    let t4948 = t1386 * t515;
    let t4961 = t1552 * t200;
    let t4962 = t4961 * t172;
    let t4978 = t21 * t5;
    (t4925, t4939, t4940, t4948, t4962, t4978)
}
