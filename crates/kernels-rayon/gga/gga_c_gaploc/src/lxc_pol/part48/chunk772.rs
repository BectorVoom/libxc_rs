//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 772/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk772(t107: f64, t37975: f64, t544: f64, t1359: f64, t3516: f64, t3529: f64, t11271: f64, t524: f64, t11218: f64, t555: f64, t188: f64, t12380: f64, t455: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t37977 = t544 * t37975 * t107;
    let t38019 = t1359 * t3516;
    let t38051 = t1359 * t3529;
    let t38181 = t524 * t11271;
    let t38184 = t555 * t11218;
    let t38185 = t188 * t38184;
    let t39622 = t12380 * t455;
    (t37977, t38019, t38051, t38181, t38184, t38185, t39622)
}
