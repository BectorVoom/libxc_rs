//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2718/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2718(t12915: f64, t17344: f64, t20747: f64, t247: f64, t1261: f64, t44693: f64, t6421: f64, t12910: f64, t12916: f64, t20857: f64, t1208: f64, t21332: f64) -> (f64, f64, f64, f64) {
    let t70129 = t17344 * t247 * t12915 * t20747;
    let t70133 = t1261 * t247 * t44693 * t6421;
    let t70140 = t12910 * t12916 * t20857;
    let t70208 = t21332 * t1208;
    (t70129, t70133, t70140, t70208)
}
