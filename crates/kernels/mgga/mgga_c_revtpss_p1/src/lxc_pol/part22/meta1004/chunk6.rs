//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3434/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3434<F: Float>(t11466: F, t2988: F, t3012: F, t311: F, t41238: F, t41658: F, t6189: F, t6190: F, t6206: F, t63892: F, t64327: F, t64329: F, t64335: F, t64338: F, t64340: F, t64342: F, t64344: F, t64346: F, t64404: F, t64416: F, t64430: F, t64444: F, t64458: F, t64465: F, t64467: F, t64471: F, t64475: F, t64483: F) -> F {
    let t64484 = -t64327 + t64329 + F::cast_from(0.91082604192152556044e5_f64) * t41658 * t6189 * t41238 * t2988 - t64335 - t64338 - t64340 - t64342 - t64344 + t64346 + t64404 - F::new(0.310907e-1) * (t64416 + t64430 + t64444 + t64458) * t311 - t64465 - t64467 - F::cast_from(0.19751673498613801407e-1_f64) * t63892 - t64471 - t64475 + F::cast_from(0.35089341735807877242e1_f64) * t3012 * t6206 * t2988 - F::cast_from(0.14035736694323150897e2_f64) * t11466 * t6190 * t2988 - t64483;
    t64484
}
