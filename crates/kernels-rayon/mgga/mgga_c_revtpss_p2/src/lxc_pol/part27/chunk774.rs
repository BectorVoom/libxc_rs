//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 774/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk774(t162: f64, t9348: f64, t9361: f64, t187: f64, t2490: f64, t737: f64, t2492: f64, t744: f64) -> (f64, f64, f64, f64) {
    let t9363 = (t9348 + t9361) * t162;
    let t9365 = 0.19751673498613801407e-1_f64 * t9363 * t187;
    let t9367 = 1.0_f64 / t2490 / t737;
    let t9368 = t2492 * t744;
    (t9363, t9365, t9367, t9368)
}
