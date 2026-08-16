//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3276/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3276(t10777: f64, t14671: f64, t14686: f64, t4424: f64, t61956: f64, t837: f64, t18477: f64, t50769: f64, t51133: f64, t18348: f64, t2710: f64, t2713: f64) -> (f64, f64, f64, f64) {
    let t62236 = t10777 * t14686 * t14671 * t4424;
    let t62241 = t10777 * t14686 * t61956 * t837;
    let t62246 = t51133 * t50769 * t18477;
    let t62251 = t2710 * t2713 * t18348;
    (t62236, t62241, t62246, t62251)
}
