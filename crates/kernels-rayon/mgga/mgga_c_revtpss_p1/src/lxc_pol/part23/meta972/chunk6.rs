//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3297/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3297(t5741: f64, t75251: f64, t47348: f64, t47351: f64, t47352: f64, t47381: f64, t49290: f64, t75174: f64, t75176: f64, t75179: f64, t75190: f64, t75205: f64) -> f64 {
    let t86563 = t75251 * t5741;
    let t86567 = -0.16463622957338778996e-1_f64 * t75174 + 0.7805952431506226415e-1_f64 * t75176 - 0.7805952431506226415e-1_f64 * t75179 + 0.19637199382202157274e-3_f64 * t47348 - t47351 + 0.26019841438354088051e-2_f64 * t47352 - 0.65854491829355115984e-1_f64 * t75190 - t49290 - 0.29272321618148349057e-1_f64 * t86563 - 0.11044544084478153697e-3_f64 * t47381 + 0.16463622957338778996e-1_f64 * t75205;
    t86567
}
