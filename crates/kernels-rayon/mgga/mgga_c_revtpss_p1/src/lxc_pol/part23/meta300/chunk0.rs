//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1552/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1552(t1052: f64, t3147: f64, t1036: f64, t3141: f64, t3144: f64, t1035: f64, t11239: f64) -> (f64, f64, f64, f64, f64) {
    let t11997 = t1052 * t3147;
    let t11998 = t1036 * t11997;
    let t11999 = t3141 * t11998;
    let t12012 = t3144 * t11997;
    let t12013 = t3141 * t12012;
    let t12046 = t11239 * t1035;
    (t11998, t11999, t12012, t12013, t12046)
}
