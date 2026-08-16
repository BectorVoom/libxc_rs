//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 801/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk801(t10177: f64, t19531: f64, t883: f64, t9074: f64, t10171: f64, t2317: f64, t6525: f64, t2321: f64, t34478: f64, t123: f64, t31730: f64, t2326: f64) -> (f64, f64, f64, f64) {
    let t42651 = t9074 * t19531 * t883 * t10177;
    let t42661 = t6525 * t10171 * t2317;
    let t42664 = t9074 * t34478 * t2321;
    let t42669 = t31730 * t123;
    let t42671 = t9074 * t42669 * t2326;
    (t42651, t42661, t42664, t42671)
}
