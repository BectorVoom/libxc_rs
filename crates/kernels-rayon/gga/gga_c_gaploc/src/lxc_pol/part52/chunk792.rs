//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 792/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk792(t6520: f64, t6525: f64, t7888: f64, t2326: f64, t3394: f64, t6514: f64, t9074: f64, t30204: f64, t31769: f64, t10177: f64, t19531: f64, t883: f64) -> (f64, f64, f64, f64) {
    let t42640 = t6525 * t7888 * t6520;
    let t42644 = t9074 * t6514 * t3394 * t2326;
    let t42647 = t9074 * t30204 * t31769;
    let t42651 = t9074 * t19531 * t883 * t10177;
    (t42640, t42644, t42647, t42651)
}
