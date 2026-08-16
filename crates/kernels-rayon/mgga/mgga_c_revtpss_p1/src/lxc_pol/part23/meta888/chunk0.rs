//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2812/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2812(t10529: f64, t2782: f64, t76106: f64, t233: f64, t23359: f64, t689: f64, t869: f64, t14598: f64, t23160: f64, t686: f64, t72: f64, t23244: f64, t251: f64) -> (f64, f64, f64, f64) {
    let t76108 = t2782 * t10529 * t76106;
    let t76117 = t689 * t869 * t233 * t23359;
    let t76125 = t14598 * t23160 * t72 * t686;
    let t76127 = t251 * t23244;
    (t76108, t76117, t76125, t76127)
}
