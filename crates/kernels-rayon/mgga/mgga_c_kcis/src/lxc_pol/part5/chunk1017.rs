//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1017/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1017(t3255: f64, t4639: f64, t4644: f64, t1035: f64, t1670: f64, t4572: f64, t1098: f64, t4627: f64, t41: f64, t85: f64, t8565: f64, t4589: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14202 = 0.19711289e-2_f64 * t3255 * t4639;
    let t14204 = 0.26281718666666666666e-2_f64 * t3255 * t4644;
    let t14215 = t1035 * t1670;
    let t14232 = 0.13140859333333333334e-2_f64 * t3255 * t4572;
    let t14235 = 0.19711289e-2_f64 * t1098 * t4627;
    let t14249 = t85 * t8565 * t41;
    let t14250 = t14249 * t4589;
    (t14202, t14204, t14215, t14232, t14235, t14249, t14250)
}
