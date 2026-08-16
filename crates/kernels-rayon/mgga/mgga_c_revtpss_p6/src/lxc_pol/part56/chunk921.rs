//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 921/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk921(t1940: f64, t2403: f64, t30: f64, t31859: f64, t31863: f64, t31873: f64, t31876: f64, t605: f64, t7010: f64, t7091: f64, t7092: f64, t8490: f64, t8494: f64) -> f64 {
    let t31882 = 3.0_f64 / 2.0_f64 * t2403 * t8490 * t7010 + t1940 * t31859 * t30 / 2.0_f64 - t1940 * t31863 * t7092 / 2.0_f64 + t1940 * t8490 * t605 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t2403 * t8494 * t7010 - t1940 * t7091 * t31873 + t1940 * t31876 * t7092 - t1940 * t8494 * t605 / 2.0_f64;
    t31882
}
