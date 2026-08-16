//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1204/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1204(t1936: f64, t670: f64, t4147: f64, t7311: f64, t11239: f64, t3268: f64, t2645: f64, t4366: f64, t837: f64, t211: f64, t9644: f64, t138: f64, t785: f64, t9302: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t28264 = t670 * t1936;
    let t32113 = t4147 * t7311;
    let t36870 = t11239 * t3268;
    let t39588 = t4366 * t2645;
    let t39620 = t837 * t2645;
    let t39643 = 1.0_f64 / t9644 / t211;
    let t40270 = t138 * t9302 * t785;
    (t28264, t32113, t36870, t39588, t39620, t39643, t40270)
}
