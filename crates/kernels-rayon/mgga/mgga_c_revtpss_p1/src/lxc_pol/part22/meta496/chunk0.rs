//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2227/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2227(t16313: f64, t3066: f64, t1695: f64, t3325: f64, t3269: f64, t3270: f64, t11121: f64, t5015: f64, t999: f64, t1079: f64, t342: f64, t4930: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t16314 = t16313 * t3066;
    let t16317 = t1695 * t3325;
    let t16318 = t3269 * t16317;
    let t16321 = t1695 * t3270;
    let t16322 = t11121 * t16321;
    let t16327 = t5015 * t999;
    let t16328 = t1079 * t16327;
    let t16333 = t342 * t4930;
    (t16314, t16317, t16318, t16321, t16322, t16327, t16328, t16333)
}
