//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1116/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1116(t1419: f64, t7063: f64, t25081: f64, t7234: f64, t606: f64, t68: f64, t198: f64, t206: f64, t7427: f64, t11064: f64, t1892: f64, t7897: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t94801 = t7063 * t1419;
    let t95088 = t7234 * t25081;
    let t95334 = t606 * t68;
    let t95511 = t198 * t206 * t7427;
    let t95976 = t7427 * t11064;
    let t98040 = t7063 * t1892;
    let t98450 = t7897 * t25081;
    (t94801, t95088, t95334, t95511, t95976, t98040, t98450)
}
