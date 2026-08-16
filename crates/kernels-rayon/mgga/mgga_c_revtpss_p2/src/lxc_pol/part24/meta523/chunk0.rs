//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1553/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1553(t1261: f64, t12884: f64, t24232: f64, t247: f64, t1263: f64, t24616: f64, t24633: f64, t17525: f64, t21188: f64, t24758: f64, t3172: f64, t3711: f64) -> (f64, f64, f64, f64, f64) {
    let t82757 = t1261 * t247 * t12884 * t24232;
    let t82799 = t1263 * t24616;
    let t82816 = t1263 * t24633;
    let t82821 = t17525 * t21188;
    let t82824 = t3711 * t3172 * t24758;
    (t82757, t82799, t82816, t82821, t82824)
}
