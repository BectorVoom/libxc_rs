//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3171/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3171(t1180: f64, t1188: f64, t12494: f64, t12497: f64, t17097: f64, t17151: f64, t3454: f64, t3480: f64, t3491: f64, t58317: f64, t58322: f64, t58325: f64, t58327: f64, t58330: f64, t58333: f64, t58336: f64, t58341: f64, t58344: f64, t58345: f64, t58456: f64, t58462: f64, t58464: f64) -> f64 {
    let t58465 = 0.96491876992155210402e2_f64 * t58317 * t3480 + t58322 - t58325 - t58327 - t58330 - t58333 + 0.35089341735807877242e1_f64 * t17097 * t12494 - 6.0_f64 * t58336 * t3454 + t58341 + t58344 + 0.10526802520742363173e2_f64 * t58345 * t12497 + 0.17544670867903938621e1_f64 * t3491 * t17151 + 0.5848223622634646207e0_f64 * t1180 * t58456 * t1188 - t58462 - t58464;
    t58465
}
