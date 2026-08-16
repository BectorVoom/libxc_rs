//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2307/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2307(t21013: f64, t3782: f64, t12712: f64, t471: f64, t1774: f64, t3367: f64, t17934: f64, t5330: f64, t1248: f64, t3604: f64, t3670: f64, t5390: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21017 = t3782 * t21013;
    let t21028 = t12712 * t471;
    let t21035 = t1774 * t3367;
    let t21049 = t17934 * t5330;
    let t21119 = t3604 * t1248;
    let t21203 = t3670 * t5390;
    (t21017, t21028, t21035, t21049, t21119, t21203)
}
