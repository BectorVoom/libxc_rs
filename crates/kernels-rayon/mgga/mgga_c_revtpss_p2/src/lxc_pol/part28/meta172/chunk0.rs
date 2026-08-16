//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 882/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk882(t1280: f64, t3568: f64, t1284: f64, t487: f64, t1209: f64, t1287: f64, t3721: f64, t1269: f64, t473: f64, t1214: f64, t3584: f64, t3140: f64, t3596: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3751 = t1280 * t3568;
    let t3754 = t1284 * t487;
    let t3755 = t1209 * t3754;
    let t3756 = t3721 * t1287;
    let t3759 = t473 * t1269;
    let t3760 = t3759 * t1214;
    let t3763 = t1280 * t3584;
    let t3766 = t3140 * t3596;
    (t3751, t3754, t3755, t3756, t3759, t3760, t3763, t3766)
}
