//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1063/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1063(t36367: f64, t36372: f64, t36377: f64, t36380: f64, t36382: f64, t36388: f64, t2138: f64, t2147: f64, t2394: f64, t879: f64, t33524: f64, t639: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t37980 = t36367 / 24.0_f64;
    let t37983 = 0.68598428988911579156e-2_f64 * t36372;
    let t37985 = 0.21437009059034868486e-2_f64 * t36377;
    let t37987 = 7.0_f64 / 72.0_f64 * t36380;
    let t37988 = 0.5603125e-1_f64 * t36382;
    let t37992 = 0.68598428988911579156e-2_f64 * t36388;
    let t38008 = t2138 * t2147 * t2394 * t879;
    let t38010 = t33524 * t639;
    (t37980, t37983, t37985, t37987, t37988, t37992, t38008, t38010)
}
