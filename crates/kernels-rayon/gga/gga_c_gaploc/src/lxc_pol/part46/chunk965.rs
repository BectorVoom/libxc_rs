//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 965/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk965(t33399: f64, t959: f64, t13118: f64, t15362: f64, t2365: f64, t32357: f64, t6111: f64, t32436: f64, t24501: f64, t825: f64, t9438: f64, t13038: f64, t2194: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43462 = t33399 * t959;
    let t43464 = t15362 * t13118;
    let t43465 = 0.59584149919750711116e-1_f64 * t43464;
    let t43467 = t6111 * t2365 * t32357;
    let t43468 = 0.59584149919750711116e-1_f64 * t43467;
    let t43470 = t6111 * t2365 * t32436;
    let t43471 = 0.59584149919750711116e-1_f64 * t43470;
    let t43476 = t825 * t9438 * t24501;
    let t43477 = 0.31952438294933958064e-1_f64 * t43476;
    let t43479 = 0.92023022289409799224e1_f64 * t2194 * t13038;
    (t43462, t43465, t43468, t43471, t43477, t43479)
}
