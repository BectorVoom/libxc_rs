//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 943/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk943<F: Float>(t36351: F, t36353: F, t36355: F, t36364: F, t36367: F, t36372: F, t36377: F, t36380: F, t36382: F, t36388: F, t2138: F, t2147: F, t2394: F, t879: F, t33524: F, t639: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t37971 = 0.25724410870841842184e-2 * t36351;
    let t37972 = 0.672375e0 * t36353;
    let t37973 = 0.3361875e0 * t36355;
    let t37979 = t36364 / 16.0;
    let t37980 = t36367 / 24.0;
    let t37983 = 0.68598428988911579156e-2 * t36372;
    let t37985 = 0.21437009059034868486e-2 * t36377;
    let t37987 = 7.0 / 72.0 * t36380;
    let t37988 = 0.5603125e-1 * t36382;
    let t37992 = 0.68598428988911579156e-2 * t36388;
    let t38008 = t2138 * t2147 * t2394 * t879;
    let t38010 = t33524 * t639;
    (t37971, t37972, t37973, t37979, t37980, t37983, t37985, t37987, t37988, t37992, t38008, t38010)
}
