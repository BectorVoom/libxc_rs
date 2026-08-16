//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1556/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1556(t12077: f64, t342: f64, t12051: f64, t3154: f64, t3298: f64, t989: f64, t4980: f64, t994: f64) -> (f64, f64, f64, f64) {
    let t12078 = t342 * t12077;
    let t12079 = t12051 * t3154;
    let t12116 = t989 * t3298;
    let t12122 = t994 * t4980;
    (t12078, t12079, t12116, t12122)
}
