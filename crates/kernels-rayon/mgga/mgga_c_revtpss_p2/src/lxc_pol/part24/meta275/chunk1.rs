//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1049/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1049(t18622: f64, t2484: f64, t125: f64, t5962: f64, t10779: f64, t14671: f64, t6035: f64, t10777: f64, t251: f64, t5977: f64, t1558: f64, t1568: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18623 = t2484 * t18622;
    let t18627 = t125 * t5962;
    let t18643 = t10779 * t14671 * t6035;
    let t18644 = t10777 * t18643;
    let t18677 = t251 * t5977;
    let t18681 = t1568 * t1558;
    (t18623, t18627, t18643, t18644, t18677, t18681)
}
