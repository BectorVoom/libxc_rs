//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 727/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk727(t12962: f64, t12969: f64, t12989: f64, t12992: f64, t12998: f64, t13808: f64, t13811: f64, t13815: f64, t13820: f64, t13824: f64, t13828: f64, t13832: f64) -> f64 {
    let t14477 = -0.21450293971110256002e1_f64 * t13808 + 0.14300195980740170668e1_f64 * t13811 - 0.13803453343411469884e2_f64 * t13815 + t12962 - 0.89376224879626066674e-1_f64 * t12969 - t13820 - t13824 + t13828 + t13832 + t12989 + t12992 + t12998;
    t14477
}
