//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 752/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk752(t1583: f64, t1940: f64, t198: f64, t2403: f64, t7091: f64, t7847: f64, t7850: f64, t892: f64, t1544: f64, t33: f64) -> (f64, f64) {
    let t7855 = -t1583 * t1940 * t7091 + t198 * t7850 * t892 + 3.0_f64 * t2403 * t7847;
    let t7862 = t33 * t1544;
    (t7855, t7862)
}
