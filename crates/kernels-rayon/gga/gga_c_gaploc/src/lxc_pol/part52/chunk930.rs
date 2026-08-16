//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 930/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk930(t10348: f64, t11362: f64, t11429: f64, t1415: f64, t7030: f64, t13471: f64, t7014: f64, t2898: f64, t44310: f64, t900: f64, t13415: f64, t4950: f64) -> (f64, f64, f64, f64, f64) {
    let t46724 = 0.7150097990370085334e0_f64 * t11362 * t10348;
    let t46729 = t1415 * t11429 * t7030;
    let t46730 = 0.14896037479937677779e-1_f64 * t46729;
    let t46731 = t7014 * t13471;
    let t46732 = 0.19171462976960374838e0_f64 * t46731;
    let t46734 = t2898 * t900 * t44310;
    let t46735 = 0.29792074959875355558e-1_f64 * t46734;
    let t46740 = 0.71500979903700853338e0_f64 * t4950 * t13415;
    (t46724, t46730, t46732, t46735, t46740)
}
