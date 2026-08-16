//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 998/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk998(t1264: f64, t20552: f64, t2143: f64, t26341: f64, t30404: f64, t30437: f64, t30442: f64, t30445: f64, t30448: f64, t30452: f64, t30465: f64, t361: f64, t4031: f64, t4081: f64, t6095: f64, t7928: f64, t7978: f64, t7995: f64) -> f64 {
    let t30474 = 0.1038945353962551798e3_f64 * t1264 * t30404 - 0.58482233974552040708e0_f64 * t1264 * t30437 - 6.0_f64 * t20552 * t7928 + 6.0_f64 * t4081 * t30442 - 6.0_f64 * t4031 * t30445 + 0.48245472966453314466e2_f64 * t4081 * t30448 - 0.51947267698127589897e2_f64 * t1264 * t30452 - 0.62182e-1_f64 * t30465 * t361 - 0.17544670192365612213e1_f64 * t26341 * t2143 - 0.17544670192365612213e1_f64 * t6095 * t7995 + 0.35089340384731224426e1_f64 * t6095 * t7978;
    t30474
}
