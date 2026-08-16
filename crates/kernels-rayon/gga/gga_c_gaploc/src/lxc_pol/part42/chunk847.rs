//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 847/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk847(t11588: f64, t2508: f64, t954: f64, t13545: f64, t7129: f64, t2586: f64, t3650: f64, t13492: f64, t7137: f64, t11613: f64, t7696: f64, t11608: f64, t2530: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t45065 = 0.76905262301422242837e-2_f64 * t2508 * t954 * t11588;
    let t45072 = 0.53833683610995569986e-1_f64 * t7129 * t13545;
    let t45077 = 0.53833683610995569986e-1_f64 * t2508 * t3650 * t2586;
    let t45079 = 0.12304841968227558854e0_f64 * t7137 * t13492;
    let t45083 = 0.92286314761706691403e-1_f64 * t7129 * t13492;
    let t45086 = 0.92286314761706691403e-1_f64 * t2508 * t11613 * t7696;
    let t45087 = t11608 * t2530;
    (t45065, t45072, t45077, t45079, t45083, t45086, t45087)
}
