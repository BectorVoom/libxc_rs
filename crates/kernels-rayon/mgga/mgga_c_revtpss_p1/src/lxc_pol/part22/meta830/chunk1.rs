//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2951/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2951(t13848: f64, t47274: f64, t9816: f64, t9956: f64, t13878: f64, t9765: f64, t13869: f64, t3989: f64, t2661: f64, t5608: f64, t9840: f64, t9934: f64) -> (f64, f64, f64, f64) {
    let t48498 = t9816 * t47274 * t13848 * t9956;
    let t48508 = t9765 * t13878;
    let t48510 = t3989 * t13869;
    let t48514 = t2661 * t9934 * t5608 * t9840;
    (t48498, t48508, t48510, t48514)
}
