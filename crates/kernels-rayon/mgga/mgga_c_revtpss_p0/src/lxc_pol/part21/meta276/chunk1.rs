//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1502/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1502(t10061: f64, t2782: f64, t10059: f64, t4086: f64, t543: f64, t123: f64, t212: f64, t2434: f64) -> (f64, f64, f64, f64) {
    let t10062 = t2782 * t10061;
    let t10065 = t4086 * t10059 * t543;
    let t10066 = t2782 * t10065;
    let t10069 = t123 * t2434 * t212;
    (t10062, t10065, t10066, t10069)
}
