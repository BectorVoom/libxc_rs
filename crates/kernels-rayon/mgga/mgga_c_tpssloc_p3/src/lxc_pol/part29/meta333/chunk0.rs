//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1392/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1392(t11716: f64, t11717: f64, t11713: f64, t3508: f64, t475: f64, t3503: f64, t11708: f64, t3514: f64, t1210: f64, t248: f64, t3509: f64, t3570: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11718 = t11716 * t11717;
    let t11719 = t11713 * t11718;
    let t11721 = t3508 * t475;
    let t11727 = t3503 * t11717;
    let t11728 = t11713 * t11727;
    let t11734 = t11708 * t3514;
    let t11737 = t1210 * t11717;
    let t11738 = t11713 * t11737;
    let t11745 = t248 * t3570 * t3509;
    (t11719, t11721, t11728, t11734, t11738, t11745)
}
