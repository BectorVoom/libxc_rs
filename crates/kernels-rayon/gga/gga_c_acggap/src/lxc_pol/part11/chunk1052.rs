//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1052/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1052(t4447: f64, t7561: f64, t4384: f64, t8511: f64, t30371: f64, t4376: f64, t4380: f64, t2068: f64, t7422: f64, t8480: f64, t2264: f64, t30456: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34457 = t7561 * t4447;
    let t34459 = t8511 * t4384;
    let t34461 = t30371 * t4376;
    let t34463 = t8511 * t4380;
    let t34466 = t2068 * t8480 * t7422;
    let t34468 = t30456 * t2264;
    (t34457, t34459, t34461, t34463, t34466, t34468)
}
