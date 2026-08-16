//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 920/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk920(t11116: f64, t2924: f64, t1077: f64, t225: f64, t1096: f64, t3270: f64, t1071: f64, t3046: f64, t268: f64, t271: f64, t7021: f64) -> (f64, f64, f64, f64) {
    let t11118 = 0.48245938496077605201e2_f64 * t2924 * t11116;
    let t11119 = t1077 * t1077;
    let t11120 = 1.0_f64 / t11119;
    let t11121 = t225 * t11120;
    let t11122 = t3270 * t1096;
    let t11123 = t11121 * t11122;
    let t11128 = t3046 * t1071;
    let t11132 = t268 * t7021 * t271;
    (t11118, t11123, t11128, t11132)
}
