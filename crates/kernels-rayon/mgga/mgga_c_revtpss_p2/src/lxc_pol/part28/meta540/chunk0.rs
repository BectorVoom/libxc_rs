//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1989/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1989(t4147: f64, t7311: f64, t1925: f64, t36: f64, t606: f64, t7933: f64, t1450: f64, t11239: f64, t3268: f64, t211: f64, t9644: f64, t138: f64, t785: f64, t9302: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t32113 = t4147 * t7311;
    let t32591 = t1925 * t36;
    let t32592 = t32591 * t606;
    let t33651 = t4147 * t7933;
    let t35070 = t7311 * t1450;
    let t36870 = t11239 * t3268;
    let t39643 = 1.0_f64 / t9644 / t211;
    let t40270 = t138 * t9302 * t785;
    (t32113, t32592, t33651, t35070, t36870, t39643, t40270)
}
