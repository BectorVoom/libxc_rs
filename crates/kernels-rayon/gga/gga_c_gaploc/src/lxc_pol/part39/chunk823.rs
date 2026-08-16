//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 823/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk823(t3234: f64, t701: f64, t2610: f64, t22542: f64, t822: f64, t2021: f64, t6109: f64, t899: f64, t1858: f64, t3209: f64, t107: f64, t408: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28236 = t3234 * t701;
    let t28302 = t2610 * t28236;
    let t28309 = t822 * t22542;
    let t28412 = t2021 * t6109 * t899;
    let t28431 = t1858 * t3209;
    let t28438 = t107 * t408;
    (t28236, t28302, t28309, t28412, t28431, t28438)
}
