//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 884/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk884(t1023: f64, t10426: f64, t4582: f64, t1005: f64, t3082: f64, t1004: f64, t3088: f64, t1036: f64, t3094: f64, t1929: f64, t35: f64, t364: f64) -> (f64, f64, f64, f64, f64) {
    let t10432 = t10426 * t1023;
    let t10433 = t4582 * t10432;
    let t10436 = t1005 * t3082;
    let t10438 = t1004 * t3088;
    let t10441 = t3094 * t1036;
    let t10444 = 1.0_f64 / t35 / t1929;
    let t10445 = t364 * t10444;
    (t10433, t10436, t10438, t10441, t10445)
}
