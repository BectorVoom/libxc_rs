//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2942/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2942(t13793: f64, t13999: f64, t1868: f64, t3923: f64, t13872: f64, t221: f64, t3978: f64, t9921: f64, t1320: f64, t13632: f64, t13672: f64, t3860: f64, t5567: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t48111 = t13999 * t13793;
    let t48113 = t1868 * t3923;
    let t48141 = t221 * t13872;
    let t48143 = t3978 * t9921 * t48141;
    let t48152 = t1320 * t13632;
    let t48154 = t1320 * t13672;
    let t48158 = t3860 * t5567;
    (t48111, t48113, t48143, t48152, t48154, t48158)
}
