//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1517/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1517(t2723: f64, t2782: f64, t4503: f64, t76169: f64, t14568: f64, t18726: f64, t10871: f64, t14545: f64, t231: f64, t2783: f64, t76127: f64, t23359: f64, t822: f64) -> (f64, f64, f64, f64, f64) {
    let t77177 = t2782 * t4503 * t76169 * t2723;
    let t77183 = t14568 * t18726;
    let t77191 = t2782 * t14545 * t76169 * t10871;
    let t77197 = t2782 * t2783 * t76127 * t231;
    let t77225 = t822 * t23359;
    (t77177, t77183, t77191, t77197, t77225)
}
