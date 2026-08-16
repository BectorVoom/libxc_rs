//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1818/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1818(t4147: f64, t7535: f64, t36: f64, t68: f64, t606: f64, t8107: f64, t1450: f64, t211: f64, t9644: f64, t138: f64, t785: f64, t9302: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33183 = t4147 * t7535;
    let t33268 = t68 * t36;
    let t33269 = t33268 * t606;
    let t34495 = t4147 * t8107;
    let t35312 = t7535 * t1450;
    let t39643 = 1.0_f64 / t9644 / t211;
    let t40270 = t138 * t9302 * t785;
    (t33183, t33269, t34495, t35312, t39643, t40270)
}
