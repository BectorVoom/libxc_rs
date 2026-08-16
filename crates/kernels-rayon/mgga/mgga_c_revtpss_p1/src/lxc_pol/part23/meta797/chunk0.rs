//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2620/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2620(t10811: f64, t18639: f64, t10905: f64, t18507: f64, t10777: f64, t10779: f64, t2749: f64, t61715: f64, t18651: f64, t14923: f64, t18456: f64, t14671: f64, t14686: f64, t14931: f64, t18632: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t62162 = t10811 * t18639;
    let t62168 = t10905 * t18507;
    let t62176 = t10777 * t10779 * t61715 * t2749;
    let t62178 = t10811 * t18651;
    let t62188 = t14923 * t18456;
    let t62216 = t14931 * t14686 * t14671 * t18632;
    (t62162, t62168, t62176, t62178, t62188, t62216)
}
