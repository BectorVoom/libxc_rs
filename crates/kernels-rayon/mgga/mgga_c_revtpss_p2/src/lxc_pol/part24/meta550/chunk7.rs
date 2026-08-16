//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1633/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1633(t6075: f64, t1544: f64, t198: f64, t207: f64, t2403: f64, t2411: f64, t39537: f64, t39540: f64, t39741: f64, t39744: f64, t39747: f64, t39750: f64, t39756: f64, t39760: f64, t39764: f64, t77460: f64, t87318: f64, t87342: f64, t87357: f64, t87373: f64, t87640: f64, t87920: f64, t892: f64) -> f64 {
    let t87926 = t6075 * t6075;
    let t87931 = t39537 - t39540 + t39741 + t39744 + t39747 + t87318 + t39750 + t39756 + t39760 + 12.0_f64 * t2403 * t77460 * t1544 - t39764 + t198 * t207 * (t87342 + t87357 + t87373 + t87920) * t892 - 3.0_f64 * t198 * t207 * t87926 * t2411 + t87640;
    t87931
}
