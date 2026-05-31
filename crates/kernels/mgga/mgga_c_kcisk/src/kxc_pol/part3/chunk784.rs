//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 784/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk784<F: Float>(t1965: F, t5365: F, t1961: F, t5397: F, t10773: F, t12066: F, t12070: F, t12073: F, t12077: F, t12081: F, t12084: F, t12087: F, t12090: F, t12095: F, t12098: F, t12099: F, t12102: F, t12105: F, t12106: F, t1975: F, t1979: F, t1980: F, t5368: F, t5373: F, t5393: F, t5398: F, t5401: F, t5405: F, t5408: F, t5409: F, t5412: F, t5415: F, t5416: F) -> F {
    let t12109 = t5365 * t1965;
    let t12114 = t1961 * t5397;
    let t12117 = -F::cast_from(6.0_f64) * t5373 * t12066 + F::cast_from(0.96494049533612093922e2_f64) * t5398 * t12070 - F::cast_from(0.35089340384731224426e1_f64) * t5408 * t12073 + F::cast_from(0.51947267698127589897e2_f64) * t5415 * t12077 - F::cast_from(0.19751789702565206229e-1_f64) * t10773 + F::cast_from(6.0_f64) * t5398 * t12081 - F::cast_from(0.35089340384731224426e1_f64) * t12084 * t5409 + F::cast_from(0.35089340384731224426e1_f64) * t5415 * t12087 + F::cast_from(0.17544670192365612213e1_f64) * t12090 * t1980 + F::cast_from(0.17544670192365612213e1_f64) * t5405 * t5412 + F::cast_from(0.51947267698127589899e2_f64) * t12095 * t5416 - F::cast_from(0.1038945353962551798e3_f64) * t12098 * t12099 + F::cast_from(0.58482233974552040708e0_f64) * t1979 * t12102 + F::cast_from(0.1025389702100779493e4_f64) * t12105 * t12106 + F::cast_from(3.0_f64) * t12109 * t1975 + F::cast_from(3.0_f64) * t5368 * t5393 + F::cast_from(0.96494049533612093922e2_f64) * t12114 * t5401;
    t12117
}
