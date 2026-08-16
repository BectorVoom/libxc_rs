//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 565/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk565(t1628: f64, t3402: f64, t10215: f64, t600: f64, t568: f64, t3414: f64, t10216: f64, t531: f64, t569: f64, t3371: f64, t524: f64, t189: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10564 = t1628 * t3402;
    let t10569 = t600 * t10215;
    let t10570 = t568 * t10569;
    let t10573 = t1628 * t3414;
    let t10578 = t531 * t10216;
    let t10583 = t569 * t10215;
    let t10584 = t568 * t10583;
    let t10587 = t524 * t3371;
    let t10590 = t189 * t10215;
    (t10564, t10570, t10573, t10578, t10584, t10587, t10590)
}
