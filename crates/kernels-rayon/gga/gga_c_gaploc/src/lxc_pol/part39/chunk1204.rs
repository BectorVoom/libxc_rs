//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1204/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1204(t12068: f64, t1445: f64, t1562: f64, t2293: f64, t11986: f64, t2464: f64, t2465: f64, t587: f64, t48086: f64, t544: f64, t9562: f64, t2365: f64, t38277: f64, t4391: f64) -> (f64, f64, f64, f64) {
    let t48149 = t1562 * t1445 * t12068 * t2293;
    let t48154 = t587 * t2464 * t2465 * t11986;
    let t48156 = t544 * t48086;
    let t48157 = t48156 * t9562;
    let t48160 = t4391 * t2365 * t38277;
    (t48149, t48154, t48157, t48160)
}
