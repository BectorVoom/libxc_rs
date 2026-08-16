//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3158/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3158(t2439: f64, t5098: f64, t56248: f64, t56252: f64, t56256: f64, t58202: f64, t58207: f64, t58209: f64, t58211: f64, t58214: f64, t58217: f64, t58220: f64, t58223: f64) -> (f64, f64) {
    let t58225 = t2439 * t5098;
    let t58226 = 0.69463333333333333334e0_f64 * t58225;
    let t58227 = 0.62517e0_f64 * t58202 + 0.17215833333333333333e1_f64 * t56248 + 0.929655e1_f64 * t56252 - 0.61977e1_f64 * t56256 - 0.92617777777777777778e-1_f64 * t58207 - 0.41678000000000000001e0_f64 * t58209 - 0.125034e1_f64 * t58211 + 0.55570666666666666666e0_f64 * t58214 + 0.20839e0_f64 * t58217 + 0.187551e1_f64 * t58220 + 0.250068e1_f64 * t58223 + t58226;
    (t58225, t58227)
}
