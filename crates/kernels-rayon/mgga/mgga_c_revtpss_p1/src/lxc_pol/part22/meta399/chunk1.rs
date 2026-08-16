//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1989/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1989(t13944: f64, t5673: f64, t5675: f64, t5674: f64, t9955: f64, t9956: f64, t4000: f64, t820: f64, t844: f64) -> (f64, f64, f64) {
    let t13991 = t5673 * t13944 * t5675;
    let t13995 = t9955 * t5674 * t9956;
    let t13999 = t820 * t4000 * t844;
    (t13991, t13995, t13999)
}
