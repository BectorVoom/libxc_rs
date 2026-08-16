//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1354/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1354(t11716: f64, t11717: f64, t11713: f64, t3508: f64, t475: f64, t3503: f64, t11708: f64, t3514: f64, t1210: f64, t3247: f64, t415: f64, t121: f64, t3584: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11718 = t11716 * t11717;
    let t11719 = t11713 * t11718;
    let t11721 = t3508 * t475;
    let t11727 = t3503 * t11717;
    let t11728 = t11713 * t11727;
    let t11734 = t11708 * t3514;
    let t11737 = t1210 * t11717;
    let t11738 = t11713 * t11737;
    let t11778 = 1.0_f64 / t415 / t3247;
    let t11784 = t121 * t3584;
    (t11719, t11721, t11728, t11734, t11738, t11778, t11784)
}
