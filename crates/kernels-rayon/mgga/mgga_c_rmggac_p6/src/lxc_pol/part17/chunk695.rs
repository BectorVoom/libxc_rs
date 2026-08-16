//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 695/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk695(t675: f64, t9938: f64, t2402: f64, t558: f64, t884: f64, t1707: f64, t645: f64, t3928: f64, t2060: f64, t6522: f64, t1550: f64, t2024: f64, t6557: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9939 = t675 * t9938;
    let t9940 = 0.51077519871957407276e-4_f64 * t9939;
    let t9944 = t2402 * t558;
    let t9945 = t884 * t9944;
    let t9946 = 0.11974241701863808564e0_f64 * t9945;
    let t9948 = t645 * t1707;
    let t9949 = t3928 * t9948;
    let t9950 = 0.17961362552795712846e0_f64 * t9949;
    let t9951 = t2060 * t6522;
    let t9952 = t1550 * t9951;
    let t9953 = 0.5987120850931904282e-1_f64 * t9952;
    let t9954 = t2024 * t6557;
    (t9940, t9944, t9946, t9948, t9950, t9951, t9953, t9954)
}
