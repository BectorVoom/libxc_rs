//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 883/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk883(t13545: f64, t7129: f64, t2508: f64, t2586: f64, t3650: f64, t13492: f64, t7137: f64, t11613: f64, t7696: f64, t11608: f64, t2530: f64, t2580: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t45072 = 0.53833683610995569986e-1_f64 * t7129 * t13545;
    let t45077 = 0.53833683610995569986e-1_f64 * t2508 * t3650 * t2586;
    let t45079 = 0.12304841968227558854e0_f64 * t7137 * t13492;
    let t45083 = 0.92286314761706691403e-1_f64 * t7129 * t13492;
    let t45086 = 0.92286314761706691403e-1_f64 * t2508 * t11613 * t7696;
    let t45087 = t11608 * t2530;
    let t45090 = 0.15381052460284448567e-1_f64 * t2508 * t2580 * t45087;
    (t45072, t45077, t45079, t45083, t45086, t45087, t45090)
}
