//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 800/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk800(t3129: f64, t31903: f64, t9074: f64, t10166: f64, t9086: f64, t6520: f64, t6525: f64, t7888: f64, t2326: f64, t3394: f64, t6514: f64, t30204: f64, t31769: f64) -> (f64, f64, f64, f64, f64) {
    let t42587 = t9074 * t31903 * t3129;
    let t42590 = t9074 * t10166 * t9086;
    let t42640 = t6525 * t7888 * t6520;
    let t42644 = t9074 * t6514 * t3394 * t2326;
    let t42647 = t9074 * t30204 * t31769;
    (t42587, t42590, t42640, t42644, t42647)
}
