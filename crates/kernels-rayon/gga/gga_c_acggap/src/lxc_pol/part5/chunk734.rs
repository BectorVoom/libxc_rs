//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 734/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk734(t4128: f64, t5336: f64, t5357: f64, t5390: f64, t105: f64, t469: f64, t96: f64, t1670: f64, t1674: f64, t922: f64, t1662: f64, t814: f64) -> (f64, f64, f64, f64) {
    let t5392 = t4128 + t5336 + t5357 + t5390;
    let t5395 = t96 * t105 * t5392 * t469;
    let t5397 = t1674 * t1670 * t922;
    let t5399 = t1662 * t814;
    (t5392, t5395, t5397, t5399)
}
