//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1110/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1110(t11064: f64, t7427: f64, t116: f64, t28159: f64, t1892: f64, t7063: f64, t25081: f64, t7897: f64, t7234: f64, t8995: f64, t2: f64, t2411: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t95976 = t7427 * t11064;
    let t97622 = t28159 * t116;
    let t98040 = t7063 * t1892;
    let t98450 = t7897 * t25081;
    let t98588 = t7234 * t8995;
    let t98631 = t2411 * t2;
    (t95976, t97622, t98040, t98450, t98588, t98631)
}
