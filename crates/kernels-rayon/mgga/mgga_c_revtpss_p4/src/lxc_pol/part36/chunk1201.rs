//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1201/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1201(t1208: f64, t30881: f64, t487: f64, t1828: f64, t8190: f64, t7652: f64, t1287: f64, t1794: f64, t29122: f64, t2150: f64, t30840: f64, t473: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30882 = t30881 * t1208;
    let t30883 = t30882 * t487;
    let t30886 = t8190 * t1828;
    let t30887 = t7652 * t30886;
    let t30893 = t29122 * t1794 * t1287;
    let t30899 = t2150 * t473 * t30840;
    (t30882, t30883, t30886, t30887, t30893, t30899)
}
