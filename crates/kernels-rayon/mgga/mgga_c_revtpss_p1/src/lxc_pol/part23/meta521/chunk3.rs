//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2035/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2035(t1248: f64, t6587: f64, t1250: f64, t3720: f64, t17183: f64, t5330: f64) -> (f64, f64, f64, f64) {
    let t21298 = t6587 * t1248;
    let t21299 = t21298 * t1250;
    let t21300 = t3720 * t21299;
    let t21306 = t17183 * t5330;
    (t21298, t21299, t21300, t21306)
}
