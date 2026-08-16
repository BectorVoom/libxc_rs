//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1502/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1502(t231: f64, t2782: f64, t2783: f64, t76136: f64, t4500: f64, t62967: f64, t23168: f64, t39598: f64, t686: f64, t72: f64, t10530: f64, t23172: f64) -> (f64, f64, f64, f64) {
    let t76139 = t2782 * t2783 * t76136 * t231;
    let t76144 = t62967 * t4500;
    let t76153 = t39598 * t23168 * t72 * t686;
    let t76158 = t10530 * t23172 * t72 * t686;
    (t76139, t76144, t76153, t76158)
}
