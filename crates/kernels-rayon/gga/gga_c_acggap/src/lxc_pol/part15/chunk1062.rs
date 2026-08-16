//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1062/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1062(t36240: f64, t36273: f64, t36283: f64, t36286: f64, t36292: f64, t36299: f64, t36302: f64, t36331: f64, t36351: f64, t36353: f64, t36355: f64, t36364: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t37924 = 0.16006300097412701803e-1_f64 * t36240;
    let t37934 = 0.21437009059034868486e-2_f64 * t36273;
    let t37937 = 0.85748036236139473944e-3_f64 * t36283;
    let t37938 = 0.34299214494455789578e-1_f64 * t36286;
    let t37941 = 0.21437009059034868486e-2_f64 * t36292;
    let t37944 = 0.28582678745379824648e-2_f64 * t36299;
    let t37945 = 0.17149607247227894789e-2_f64 * t36302;
    let t37960 = 0.17149607247227894789e-2_f64 * t36331;
    let t37971 = 0.25724410870841842184e-2_f64 * t36351;
    let t37972 = 0.672375e0_f64 * t36353;
    let t37973 = 0.3361875e0_f64 * t36355;
    let t37979 = t36364 / 16.0_f64;
    (t37924, t37934, t37937, t37938, t37941, t37944, t37945, t37960, t37971, t37972, t37973, t37979)
}
