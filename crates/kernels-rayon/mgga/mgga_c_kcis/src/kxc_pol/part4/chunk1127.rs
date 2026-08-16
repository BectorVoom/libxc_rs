//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1127/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1127(t41: f64, t85: f64, t8565: f64, t4589: f64, t1109: f64, t13744: f64, t345: f64, t1098: f64, t4672: f64, t1670: f64, t3288: f64, t3270: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14249 = t85 * t8565 * t41;
    let t14250 = t14249 * t4589;
    let t14252 = t1109 * t13744;
    let t14253 = t345 * t14252;
    let t14260 = 0.13140859333333333333e-2_f64 * t1098 * t4672;
    let t14262 = t3288 * t1670;
    let t14263 = t14262 * t3270;
    (t14249, t14250, t14252, t14253, t14260, t14263)
}
