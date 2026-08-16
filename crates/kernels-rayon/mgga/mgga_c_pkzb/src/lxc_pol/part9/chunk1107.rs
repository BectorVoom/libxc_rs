//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1107/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1107(t218: f64, t6189: f64, t675: f64, t18439: f64, t16194: f64, t339: f64, t930: f64, t336: f64, t2316: f64, t2319: f64, t2294: f64, t18442: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t18457 = t218 * t675 * t6189;
    let t18468 = 280.0_f64 / 81.0_f64 * t18439;
    let t18480 = 1.0_f64 / t339 / t16194 / t930 / 96.0_f64;
    let t18492 = f64::powf(t336, -0.25e1_f64);
    let t18508 = t2316 * t2316;
    let t18509 = 1.0_f64 / t18508;
    let t18512 = t2319 * t2319;
    let t18513 = 1.0_f64 / t18512;
    let t18520 = 1.0_f64 / t2316 / t2294;
    let t18554 = 0.31003950617283950618e1_f64 * t18439;
    let t18555 = 0.13388493827160493828e1_f64 * t18442;
    (t18457, t18468, t18480, t18492, t18509, t18513, t18520, t18554, t18555)
}
