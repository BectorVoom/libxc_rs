//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2102/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2102(t1243: f64, t29109: f64, t1032: f64, t5412: f64, t2148: f64, t1276: f64, t3140: f64, t12627: f64, t7635: f64, t1770: f64, t7627: f64, t7642: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t105167 = t1243 * t29109;
    let t105202 = t5412 * t1032;
    let t105203 = t2148 * t105202;
    let t105220 = t2148 * t5412 * t3140 * t1276;
    let t105269 = t12627 * t7635 * t1276;
    let t105284 = t1770 * t7627;
    let t105350 = t7642 * t105202;
    (t105167, t105202, t105203, t105220, t105269, t105284, t105350)
}
