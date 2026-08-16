//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 828/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk828(t44266: f64, t13296: f64, t203: f64, t550: f64, t36274: f64, t4261: f64, t6525: f64, t35913: f64, t9074: f64, t19532: f64, t35959: f64, t123: f64, t37975: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t44267 = 0.71137516589190373998e-2_f64 * t44266;
    let t44268 = t203 * t13296;
    let t44269 = t550 * t44268;
    let t44277 = t6525 * t4261 * t36274;
    let t44278 = 0.23712505529730124666e-2_f64 * t44277;
    let t44280 = t9074 * t4261 * t35913;
    let t44281 = 0.47425011059460249332e-2_f64 * t44280;
    let t44283 = t9074 * t19532 * t35959;
    let t44284 = 0.71137516589190373998e-2_f64 * t44283;
    let t44285 = t37975 * t123;
    (t44267, t44268, t44269, t44278, t44281, t44284, t44285)
}
