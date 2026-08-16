//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1247/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1247(t5651: f64, t7028: f64, t9736: f64, t2689: f64, t27936: f64, t13857: f64, t94564: f64, t1885: f64, t94459: f64, t1873: f64, t94519: f64, t25240: f64, t3964: f64, t5617: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t98200 = t9736 * t7028 * t5651;
    let t98218 = t2689 * t27936;
    let t98220 = t94564 * t13857;
    let t98224 = t94459 * t1885;
    let t98260 = t94519 * t1873;
    let t98285 = t3964 * t25240 * t5617;
    (t98200, t98218, t98220, t98224, t98260, t98285)
}
