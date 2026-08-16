//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 559/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk559(t9308: f64, t9349: f64, t9388: f64, t9434: f64, t9475: f64, t9509: f64, t9551: f64, t9585: f64, t2530: f64, t2581: f64, t2580: f64, t3234: f64, t325: f64) -> (f64, f64, f64, f64) {
    let t9588 = t9308 + t9349 + t9388 + t9434 + t9475 + t9509 + t9551 + t9585;
    let t9591 = t2581 * t2530;
    let t9592 = t2580 * t9591;
    let t9595 = t325 * t3234;
    (t9588, t9591, t9592, t9595)
}
