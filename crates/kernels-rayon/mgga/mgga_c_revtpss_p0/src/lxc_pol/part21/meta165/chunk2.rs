//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1052/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1052(t2630: f64, t3869: f64, t1337: f64, t2619: f64, t514: f64) -> (f64, f64, f64) {
    let t3871 = 0.10843581300301739842e-1_f64 * t3869 * t2630;
    let t3873 = 0.24415263074675393405e-3_f64 * t1337 * t2619;
    let t3874 = 1.0_f64 / t514;
    (t3871, t3873, t3874)
}
