//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1148/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1148(t6895: f64, t888: f64, t9263: f64, t1538: f64, t20073: f64, t6583: f64, t883: f64, t4389: f64, t899: f64, t1415: f64, t6490: f64, t913: f64) -> (f64, f64, f64, f64) {
    let t30823 = t9263 * t888 * t6895;
    let t30827 = t6583 * t1538 * t883 * t20073;
    let t30829 = t4389 * t899;
    let t30830 = t1415 * t30829;
    let t30833 = 0.11916829983950142223e0_f64 * t30830 * t913 * t6490;
    (t30823, t30827, t30830, t30833)
}
