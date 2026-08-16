//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 1306/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk1306(t1049: f64, t24906: f64, t10099: f64, t12002: f64, t2469: f64, t2470: f64, t3268: f64, t338: f64, t3449: f64, t35757: f64, t35803: f64, t35845: f64, t35887: f64, t35927: f64, t35964: f64, t36008: f64, t36046: f64, t36055: f64, t36058: f64, t36067: f64, t36068: f64, t36072: f64, t36074: f64, t36078: f64, t3795: f64, t7063: f64, t9378: f64, t972: f64) -> (f64, f64) {
    let t36080 = 2.0_f64 * t24906 * t1049;
    let t36081 = (t35757 + t35803 + t35845 + t35887 + t35927 + t35964 + t36008 + t36046) * t338 - 24.0_f64 * t7063 * t3268 * t3449 + t36055 - t36058 - 6.0_f64 * t7063 * t3795 * t2470 + 4.0_f64 * t2469 * t12002 * t972 + t36067 + 4.0_f64 * t2469 * t36068 - t36072 - t36074 + 8.0_f64 * t10099 * t9378 - t36078 + t36080;
    (t36080, t36081)
}
