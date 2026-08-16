//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2530/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2530(t2439: f64, t4625: f64, t4622: f64, t123: f64, t127: f64, t159: f64) -> (f64, f64, f64, f64) {
    let t51913 = t2439 * t4625;
    let t51914 = 0.5519e0_f64 * t51913;
    let t51915 = t2439 * t4622;
    let t51957 = t123 * t127 * t159;
    (t51913, t51914, t51915, t51957)
}
