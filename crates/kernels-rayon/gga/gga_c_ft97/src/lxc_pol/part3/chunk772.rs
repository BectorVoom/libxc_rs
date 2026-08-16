//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 772/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk772(t432: f64, t4551: f64, t1852: f64, t452: f64, t3291: f64, t942: f64, t11863: f64, t15959: f64, t4431: f64, t492: f64, t1910: f64, t1909: f64) -> (f64, f64, f64, f64) {
    let t15994 = t4551 * t432;
    let t15996 = t452 * t1852 * t15994;
    let t16000 = t452 * t3291 * t942;
    let t16003 = t11863 * t15959;
    let t16006 = t4431 * t492;
    let t16007 = t1910 * t16006;
    let t16008 = t1909 * t16007;
    (t15996, t16000, t16003, t16008)
}
