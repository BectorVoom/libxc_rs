//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 460/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk460(t1568: f64, t225: f64, t257: f64, t1559: f64, t879: f64, t234: f64, t213: f64, t820: f64, t873: f64, t878: f64) -> (f64, f64, f64) {
    let t1569 = t1568 * t225;
    let t1570 = t1569 * t257;
    let t1573 = t879 * t1559;
    let t1576 = t234 * t1568;
    let t1579 = -t873 + t878 - 0.65854491829355115987e0_f64 * t820 * t1573 + 0.65854491829355115987e0_f64 * t213 * t1576;
    (t1569, t1570, t1579)
}
