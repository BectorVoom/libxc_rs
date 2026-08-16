//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1003/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1003(t35451: f64, t30984: f64, t8649: f64, t30934: f64, t8602: f64, t2450: f64, t7646: f64, t30468: f64, t4741: f64, t30216: f64, t8526: f64, t1983: f64, t30692: f64, t7586: f64, t8901: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t35452 = 0.32012600194825403606e-1_f64 * t35451;
    let t35456 = t30984 * t8649;
    let t35458 = t30934 * t8602;
    let t35459 = 0.22642626448514989489e-1_f64 * t35458;
    let t35466 = t2450 * t7646;
    let t35469 = t30468 * t4741;
    let t35471 = t30216 * t8526;
    let t35475 = t30692 * t7586 * t1983 * t8901;
    (t35452, t35456, t35459, t35466, t35469, t35471, t35475)
}
