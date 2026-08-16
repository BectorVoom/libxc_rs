//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1220/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1220(t36364: f64, t36367: f64, t36370: f64, t36372: f64, t36377: f64, t36380: f64, t36382: f64, t36388: f64, t36392: f64, t31859: f64, t31864: f64, t31870: f64, t31872: f64, t31879: f64, t32967: f64, t36374: f64, t36386: f64, t36390: f64) -> f64 {
    let t37979 = t36364 / 16.0_f64;
    let t37980 = t36367 / 24.0_f64;
    let t37982 = 0.34299214494455789578e-2_f64 * t36370;
    let t37983 = 0.68598428988911579156e-2_f64 * t36372;
    let t37985 = 0.21437009059034868486e-2_f64 * t36377;
    let t37987 = 7.0_f64 / 72.0_f64 * t36380;
    let t37988 = 0.5603125e-1_f64 * t36382;
    let t37992 = 0.68598428988911579156e-2_f64 * t36388;
    let t37994 = 0.34299214494455789578e-2_f64 * t36392;
    let t37995 = 0.85748036236139473944e-3_f64 * t31859 + t37979 + t37980 + 0.17149607247227894789e-2_f64 * t31864 - t37982 - t37983 - 0.34299214494455789578e-2_f64 * t36374 + t32967 - t37985 + t31870 / 8.0_f64 + t37987 + t37988 - 7.0_f64 / 144.0_f64 * t31872 - 0.34299214494455789578e-2_f64 * t31879 + 0.55907719625962937012e-2_f64 * t36386 + t37992 + 0.75475421495049964966e-2_f64 * t36390 + t37994;
    t37995
}
