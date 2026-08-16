//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1217/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1217(t45963: f64, t6957: f64, t10309: f64, t25105: f64, t45972: f64, t10310: f64, t77: f64, t84: f64, t2248: f64, t640: f64, t45958: f64, t10301: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t92684 = t45963 * t6957;
    let t92687 = t10309 * t25105;
    let t92690 = t45972 * t6957;
    let t92692 = t77 * t84 * t10310;
    let t92696 = t77 * t640 * t2248;
    let t92699 = t45958 * t6957;
    let t92702 = t10301 * t25105;
    (t92684, t92687, t92690, t92692, t92696, t92699, t92702)
}
