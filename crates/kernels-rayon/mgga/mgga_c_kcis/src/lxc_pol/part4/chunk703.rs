//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 703/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk703(t4059: f64, t1444: f64, t740: f64, t833: f64, t1437: f64, t3805: f64, t1330: f64, t3797: f64, t111: f64, t1404: f64, t1445: f64, t2645: f64, t4047: f64, t4050: f64, t4053: f64, t4054: f64, t4055: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4060 = 0.15538616723388920628e-3_f64 * t4059;
    let t4061 = t740 * t1444;
    let t4062 = t4061 * t833;
    let t4066 = t1437 * t3805;
    let t4069 = t1330 * t3797;
    let t4072 = t4047 - t4050 - t4053 - t4054 - 0.23911438650126355246e-1_f64 * t4055 + 0.11955719325063177623e-1_f64 * t1404 * t2645 + t4060 + 0.20718155631185227504e-3_f64 * t4062 - 0.5179538907796306876e-4_f64 * t1445 * t2645 + 0.7925e-3_f64 * t111 * t4066 - 0.52833333333333333333e-3_f64 * t111 * t4069;
    (t4060, t4061, t4062, t4066, t4069, t4072)
}
