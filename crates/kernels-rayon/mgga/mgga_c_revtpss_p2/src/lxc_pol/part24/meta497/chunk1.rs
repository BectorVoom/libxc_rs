//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1498/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1498(t18805: f64, t50208: f64, t4321: f64, t6049: f64, t689: f64, t4481: f64, t63084: f64, t1580: f64, t18316: f64, t14480: f64, t252: f64, t2782: f64, t6071: f64) -> (f64, f64, f64, f64, f64) {
    let t75984 = t50208 * t18805;
    let t75998 = t689 * t4321 * t6049;
    let t76010 = t63084 * t4481;
    let t76020 = t689 * t18316 * t1580;
    let t76026 = t2782 * t252 * t14480 * t6071;
    (t75984, t75998, t76010, t76020, t76026)
}
