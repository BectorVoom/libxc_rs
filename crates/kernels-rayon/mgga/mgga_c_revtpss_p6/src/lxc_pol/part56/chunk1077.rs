//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1077/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1077(t1121: f64, t1276: f64, t1032: f64, t26948: f64, t33424: f64, t3566: f64, t11239: f64, t2148: f64, t8931: f64, t1209: f64, t124604: f64, t3596: f64, t3736: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t124827 = t1276 * t1121;
    let t124838 = t26948 * t1032;
    let t124862 = t3566 * t1032 * t33424;
    let t124869 = t2148 * t8931 * t11239 * t1276;
    let t124887 = t1209 * t124604;
    let t124891 = t3736 * t3596;
    (t124827, t124838, t124862, t124869, t124887, t124891)
}
