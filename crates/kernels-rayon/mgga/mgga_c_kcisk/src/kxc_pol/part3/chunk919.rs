//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 919/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk919(t357: f64, t4079: f64, t346: f64, t1253: f64, t4032: f64, t4083: f64, t1255: f64, t1264: f64, t1276: f64, t13512: f64, t13518: f64, t13557: f64, t13566: f64, t13571: f64, t13574: f64, t13578: f64, t13583: f64, t361: f64, t4026: f64, t4076: f64, t4084: f64, t4096: f64, t4103: f64) -> (f64, f64) {
    let t13587 = 1.0_f64 / t4079 / t357;
    let t13588 = t346 * t13587;
    let t13589 = t4032 * t1253;
    let t13590 = t13589 * t4083;
    let t13593 = -0.17544670192365612213e1_f64 * t13512 * t1276 + 0.35089340384731224426e1_f64 * t4096 * t4103 - 0.51947267698127589897e2_f64 * t1264 * t13518 - 0.62182e-1_f64 * t13557 * t361 - 0.1025389702100779493e4_f64 * t1264 * t13566 + 0.35089340384731224426e1_f64 * t1264 * t13571 + 3.0_f64 * t13574 * t1255 - 0.35089340384731224426e1_f64 * t1264 * t13578 + 3.0_f64 * t4026 * t4076 + 0.48245472966453314466e2_f64 * t13583 * t4084 - 0.96490945932906628932e2_f64 * t13588 * t13590;
    (t13589, t13593)
}
