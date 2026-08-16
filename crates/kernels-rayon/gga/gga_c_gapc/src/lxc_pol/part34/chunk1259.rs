//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1259/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1259(t11513: f64, t5392: f64, t5395: f64, t11514: f64, t5633: f64, t137: f64, t1743: f64, t190: f64, t33235: f64, t442: f64, t5971: f64, t11484: f64, t1835: f64) -> (f64, f64, f64, f64) {
    let t34992 = t5395 * t11513 * t5392;
    let t34995 = t11514 * t5633;
    let t35001 = t1743 * t33235 * t5971 * t190 * t137 * t442;
    let t35003 = t11484 * t1835;
    (t34992, t34995, t35001, t35003)
}
