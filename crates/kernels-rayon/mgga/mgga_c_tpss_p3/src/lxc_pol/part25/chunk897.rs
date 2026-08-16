//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 897/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk897(t2202: f64, t862: f64, t235: f64, t2697: f64, t262: f64, t265: f64, t5543: f64, t599: f64, t275: f64, t277: f64, t267: f64, t270: f64, t279: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8627 = t2202 * t862;
    let t8633 = t235 * t2697;
    let t8660 = t262 * t5543 * t265;
    let t8661 = 0.93011851851851851854e0_f64 * t8660;
    let t8662 = t599 * t235;
    let t8664 = t275 * t8662 * t277;
    let t8665 = 0.36514074074074074075e0_f64 * t8664;
    let t8678 = 1.0_f64/pow_3_2(t267);
    let t8684 = 1.0_f64 / t270 / t279 / 4.0_f64;
    (t8627, t8633, t8660, t8661, t8662, t8664, t8665, t8678, t8684)
}
