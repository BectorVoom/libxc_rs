//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2609/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2609(t10777: f64, t40725: f64, t5988: f64, t837: f64, t40593: f64, t6037: f64, t125: f64, t18392: f64, t124: f64, t6016: f64) -> (f64, f64, f64, f64) {
    let t61697 = t10777 * t40725 * t5988 * t837;
    let t61699 = t40593 * t6037;
    let t61701 = t125 * t18392;
    let t61715 = t124 * t6016;
    (t61697, t61699, t61701, t61715)
}
