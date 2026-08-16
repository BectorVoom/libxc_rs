//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1031/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1031(t107: f64, t78: f64, t2032: f64, t5679: f64, t6081: f64, t1980: f64, t6110: f64, t1387: f64, t60: f64) -> (f64, f64, f64, f64, f64) {
    let t14630 = t107 * t78;
    let t14667 = t5679 * t2032;
    let t15349 = t6081 * t2032;
    let t15362 = t1980 * t6110;
    let t15478 = t60 * t1387;
    (t14630, t14667, t15349, t15362, t15478)
}
