//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1757/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1757(t2242: f64, t607: f64, t38: f64, t6972: f64, t2247: f64, t640: f64, t644: f64, t77: f64, t2315: f64, t84: f64, t2251: f64, t603: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25102 = t2242 * t607;
    let t25105 = t38 * t6972;
    let t25106 = t2247 * t25105;
    let t25110 = t77 * t640 * t644;
    let t25113 = t84 * t2315;
    let t25114 = t77 * t25113;
    let t25117 = t603 * t2251;
    (t25102, t25105, t25106, t25110, t25114, t25117)
}
