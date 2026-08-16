//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1504/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1504(t23359: f64, t686: f64, t72: f64, t874: f64, t10871: f64, t6016: f64, t4500: f64, t62808: f64, t125: f64, t23148: f64, t23167: f64, t23244: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t76237 = t874 * t23359 * t72 * t686;
    let t76242 = t10871 * t6016;
    let t76255 = t62808 * t4500;
    let t76279 = t125 * t23148;
    let t76284 = t125 * t23167;
    let t76289 = t125 * t23244;
    (t76237, t76242, t76255, t76279, t76284, t76289)
}
