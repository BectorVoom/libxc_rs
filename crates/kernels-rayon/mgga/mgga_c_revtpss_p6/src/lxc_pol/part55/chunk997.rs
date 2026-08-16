//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 997/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk997(t1248: f64, t8201: f64, t1287: f64, t8197: f64, t1209: f64, t8190: f64, t1294: f64, t7652: f64, t1770: f64, t2142: f64, t1214: f64, t7637: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t29212 = t8201 * t1248;
    let t29213 = t29212 * t1287;
    let t29216 = t8197 * t1248;
    let t29217 = t29216 * t1287;
    let t29220 = t1209 * t8190;
    let t29224 = t7652 * t8197 * t1294;
    let t29227 = t1770 * t2142;
    let t29233 = t7637 * t8190 * t1214;
    (t29213, t29217, t29220, t29224, t29227, t29233)
}
