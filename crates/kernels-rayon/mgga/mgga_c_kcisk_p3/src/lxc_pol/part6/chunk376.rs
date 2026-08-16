//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 376/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk376(t1707: f64, t2408: f64, t1714: f64, t1248: f64, t1720: f64, t2364: f64, t1712: f64, t1719: f64, t2402: f64, t1725: f64, t1729: f64, t1739: f64, t1742: f64) -> (f64, f64, f64, f64) {
    let t2409 = t1707 * t2408;
    let t2412 = t1714 * t2408;
    let t2415 = t1248 * t1720 * t2364;
    let t2417 = 0.1898925e1_f64 * t2409 - t1712 - 0.29896666666666666667e0_f64 * t2402 + 0.3071625e0_f64 * t2412 - t1719 - 0.16431333333333333333e0_f64 * t2415;
    let t2418 = t2417 * t1725;
    let t2422 = -t1729 - 0.92708333333333333333e-2_f64 * t2402;
    let t2430 = 0.258925e1_f64 * t2409 - t1739 - 0.301925e0_f64 * t2402 + 0.16504875e0_f64 * t2412 - t1742 - 0.16557e0_f64 * t2415;
    (t2417, t2418, t2422, t2430)
}
