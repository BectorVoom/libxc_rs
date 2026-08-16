//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 654/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk654(t1985: f64, t3038: f64, t926: f64, t1101: f64, t1992: f64, t1106: f64, t451: f64, t453: f64, t2719: f64, t458: f64, t2713: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3039 = t3038 * t1985;
    let t3040 = t926 * t3039;
    let t3043 = t1101 * t1992;
    let t3044 = t926 * t3043;
    let t3048 = 1.0_f64 / t1106 / t451;
    let t3049 = t3048 * t453;
    let t3050 = t458 * t2719;
    let t3052 = t2713 * t3049 * t3050;
    (t3039, t3040, t3043, t3044, t3048, t3049, t3050, t3052)
}
