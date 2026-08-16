//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1220/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1220<F: Float>(t36364: F, t36367: F, t36370: F, t36372: F, t36377: F, t36380: F, t36382: F, t36388: F, t36392: F, t31859: F, t31864: F, t31870: F, t31872: F, t31879: F, t32967: F, t36374: F, t36386: F, t36390: F) -> F {
    let t37979 = t36364 / F::cast_from(16.0_f64);
    let t37980 = t36367 / F::cast_from(24.0_f64);
    let t37982 = F::cast_from(0.34299214494455789578e-2_f64) * t36370;
    let t37983 = F::cast_from(0.68598428988911579156e-2_f64) * t36372;
    let t37985 = F::cast_from(0.21437009059034868486e-2_f64) * t36377;
    let t37987 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t36380;
    let t37988 = F::cast_from(0.5603125e-1_f64) * t36382;
    let t37992 = F::cast_from(0.68598428988911579156e-2_f64) * t36388;
    let t37994 = F::cast_from(0.34299214494455789578e-2_f64) * t36392;
    let t37995 = F::cast_from(0.85748036236139473944e-3_f64) * t31859 + t37979 + t37980 + F::cast_from(0.17149607247227894789e-2_f64) * t31864 - t37982 - t37983 - F::cast_from(0.34299214494455789578e-2_f64) * t36374 + t32967 - t37985 + t31870 / F::cast_from(8.0_f64) + t37987 + t37988 - F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t31872 - F::cast_from(0.34299214494455789578e-2_f64) * t31879 + F::cast_from(0.55907719625962937012e-2_f64) * t36386 + t37992 + F::cast_from(0.75475421495049964966e-2_f64) * t36390 + t37994;
    t37995
}
