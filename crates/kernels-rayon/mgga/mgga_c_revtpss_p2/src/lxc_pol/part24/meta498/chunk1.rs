//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1500/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1500(t23177: f64, t2798: f64, t686: f64, t72: f64, t14568: f64, t18730: f64, t14586: f64, t6016: f64, t10529: f64, t2782: f64, t233: f64, t23359: f64, t689: f64, t869: f64) -> (f64, f64, f64, f64) {
    let t76100 = t2798 * t23177 * t72 * t686;
    let t76104 = t14568 * t18730;
    let t76106 = t14586 * t6016;
    let t76108 = t2782 * t10529 * t76106;
    let t76117 = t689 * t869 * t233 * t23359;
    (t76100, t76104, t76108, t76117)
}
