//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1758/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1758(t2482: f64, t4000: f64, t596: f64, t10003: f64, t1412: f64, t3923: f64, t2661: f64, t9835: f64, t9934: f64, t9914: f64, t9918: f64, t221: f64, t4018: f64, t4019: f64, t9899: f64) -> (f64, f64, f64, f64, f64) {
    let t47215 = t2482 * t4000 * t596;
    let t47216 = t47215 * t10003;
    let t47218 = t1412 * t3923;
    let t47221 = t2661 * t9934 * t47218 * t9835;
    let t47223 = t9918 * t9914;
    let t47227 = t4018 * t4019 * t221 * t9899;
    (t47216, t47218, t47221, t47223, t47227)
}
