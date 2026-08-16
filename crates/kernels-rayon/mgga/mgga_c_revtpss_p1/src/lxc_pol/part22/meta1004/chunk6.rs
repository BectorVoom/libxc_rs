//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3434/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3434(t11466: f64, t2988: f64, t3012: f64, t311: f64, t41238: f64, t41658: f64, t6189: f64, t6190: f64, t6206: f64, t63892: f64, t64327: f64, t64329: f64, t64335: f64, t64338: f64, t64340: f64, t64342: f64, t64344: f64, t64346: f64, t64404: f64, t64416: f64, t64430: f64, t64444: f64, t64458: f64, t64465: f64, t64467: f64, t64471: f64, t64475: f64, t64483: f64) -> f64 {
    let t64484 = -t64327 + t64329 + 0.91082604192152556044e5_f64 * t41658 * t6189 * t41238 * t2988 - t64335 - t64338 - t64340 - t64342 - t64344 + t64346 + t64404 - 0.310907e-1_f64 * (t64416 + t64430 + t64444 + t64458) * t311 - t64465 - t64467 - 0.19751673498613801407e-1_f64 * t63892 - t64471 - t64475 + 0.35089341735807877242e1_f64 * t3012 * t6206 * t2988 - 0.14035736694323150897e2_f64 * t11466 * t6190 * t2988 - t64483;
    t64484
}
