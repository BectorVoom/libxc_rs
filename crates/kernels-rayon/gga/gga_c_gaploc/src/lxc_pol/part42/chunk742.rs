//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 742/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk742(t1381: f64, t2796: f64, t22537: f64, t822: f64, t2012: f64, t9804: f64, t22542: f64, t2021: f64, t6109: f64, t899: f64, t107: f64, t408: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27232 = t2796 * t1381;
    let t28069 = t822 * t22537;
    let t28073 = t2012 * t9804;
    let t28309 = t822 * t22542;
    let t28412 = t2021 * t6109 * t899;
    let t28438 = t107 * t408;
    (t27232, t28069, t28073, t28309, t28412, t28438)
}
