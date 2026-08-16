//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 784/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk784(t1965: f64, t5365: f64, t1961: f64, t5397: f64, t10773: f64, t12066: f64, t12070: f64, t12073: f64, t12077: f64, t12081: f64, t12084: f64, t12087: f64, t12090: f64, t12095: f64, t12098: f64, t12099: f64, t12102: f64, t12105: f64, t12106: f64, t1975: f64, t1979: f64, t1980: f64, t5368: f64, t5373: f64, t5393: f64, t5398: f64, t5401: f64, t5405: f64, t5408: f64, t5409: f64, t5412: f64, t5415: f64, t5416: f64) -> f64 {
    let t12109 = t5365 * t1965;
    let t12114 = t1961 * t5397;
    let t12117 = -6.0_f64 * t5373 * t12066 + 0.96494049533612093922e2_f64 * t5398 * t12070 - 0.35089340384731224426e1_f64 * t5408 * t12073 + 0.51947267698127589897e2_f64 * t5415 * t12077 - 0.19751789702565206229e-1_f64 * t10773 + 6.0_f64 * t5398 * t12081 - 0.35089340384731224426e1_f64 * t12084 * t5409 + 0.35089340384731224426e1_f64 * t5415 * t12087 + 0.17544670192365612213e1_f64 * t12090 * t1980 + 0.17544670192365612213e1_f64 * t5405 * t5412 + 0.51947267698127589899e2_f64 * t12095 * t5416 - 0.1038945353962551798e3_f64 * t12098 * t12099 + 0.58482233974552040708e0_f64 * t1979 * t12102 + 0.1025389702100779493e4_f64 * t12105 * t12106 + 3.0_f64 * t12109 * t1975 + 3.0_f64 * t5368 * t5393 + 0.96494049533612093922e2_f64 * t12114 * t5401;
    t12117
}
