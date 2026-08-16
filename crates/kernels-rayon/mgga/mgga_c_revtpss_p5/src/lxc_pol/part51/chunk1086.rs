//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1086/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1086(t5: f64, t125277: f64, t125340: f64, t117: f64, t125209: f64, t116: f64, t33629: f64, t670: f64, t8446: f64, t1936: f64, t97622: f64, t108120: f64, t28030: f64, t7002: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t125342 = piecewise3(t8, 0.0_f64, t125277 + t125340);
    let t125343 = t125342 * t117;
    let t125344 = 2.0_f64 * t125209;
    let t125345 = t33629 * t116;
    let t125350 = t8446 * t670;
    let t125355 = t97622 * t1936;
    let t125357 = t108120 * t1936;
    let t125359 = t28030 * t7002;
    (t125343, t125344, t125345, t125350, t125355, t125357, t125359)
}
