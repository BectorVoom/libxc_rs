//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1407/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1407(t11061: f64, t11064: f64, t10489: f64, t198: f64, t207: f64, t2403: f64, t2404: f64, t39528: f64, t39531: f64, t39534: f64, t39537: f64, t39540: f64, t39738: f64, t39741: f64, t39744: f64, t39747: f64, t39750: f64, t39756: f64, t40975: f64, t41023: f64, t41075: f64, t41131: f64, t775: f64, t892: f64) -> f64 {
    let t41137 = t11061 * t11064;
    let t41141 = 12.0_f64 * t2403 * t2404 * t10489 - t39528 + t39531 + t39534 + t39537 - t39540 + t198 * t207 * (t40975 + t41023 + t41075 + t41131) * t892 + 24.0_f64 * t2403 * t41137 * t775 + t39738 + t39741 + t39744 + t39747 + t39750 + t39756;
    t41141
}
