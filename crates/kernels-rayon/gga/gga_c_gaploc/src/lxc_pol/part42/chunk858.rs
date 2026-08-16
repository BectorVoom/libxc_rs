//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 858/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk858(t13593: f64, t5676: f64, t11576: f64, t2033: f64, t2365: f64, t2610: f64, t44712: f64, t701: f64, t6066: f64, t7630: f64, t36635: f64, t959: f64) -> (f64, f64, f64, f64, f64) {
    let t45299 = t5676 * t13593;
    let t45300 = 0.14896037479937677779e-1_f64 * t45299;
    let t45303 = t2033 * t2365 * t2610 * t11576;
    let t45304 = 0.14896037479937677779e-1_f64 * t45303;
    let t45305 = t44712 * t701;
    let t45308 = 0.71500979903700853338e0_f64 * t7630 * t6066 * t45305;
    let t45314 = t36635 * t959;
    (t45300, t45304, t45305, t45308, t45314)
}
