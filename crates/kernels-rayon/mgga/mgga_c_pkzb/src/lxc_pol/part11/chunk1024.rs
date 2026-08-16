//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1024/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1024(t11232: f64, t11314: f64, t237: f64, t11164: f64, t10182: f64, t3139: f64, t898: f64, t11180: f64, t6230: f64, t6233: f64, t6121: f64, t2320: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11316 = t237 * (t11232 + t11314);
    let t11318 = 0.19751673498613801407e-1_f64 * t237 * t11164;
    let t11319 = t10182 * t3139;
    let t11321 = 0.51947577317044391277e2_f64 * t898 * t11319;
    let t11322 = t6230 * t11180;
    let t11323 = t11322 * t6233;
    let t11325 = 0.10254018858216406658e4_f64 * t898 * t11323;
    let t11326 = t6121 * t11180;
    let t11327 = t11326 * t2320;
    (t11316, t11318, t11319, t11321, t11322, t11323, t11325, t11326, t11327)
}
