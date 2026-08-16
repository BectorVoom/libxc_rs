//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1160/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1160(t122820: f64, t28067: f64, t196: f64, t197: f64, t29437: f64, t2035: f64, t34399: f64, t7313: f64, t28166: f64, t8763: f64, t28168: f64, t28043: f64, t7586: f64) -> (f64, f64, f64, f64, f64) {
    let t129366 = t122820 * t28067;
    let t129370 = t29437 * t196 * t197;
    let t129371 = t129370 * t2035;
    let t129376 = t34399 * t7313;
    let t129377 = t8763 * t28166;
    let t129378 = t129377 * t28168;
    let t129395 = t7586 * t28043;
    (t129366, t129371, t129376, t129378, t129395)
}
