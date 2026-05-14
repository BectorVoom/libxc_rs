//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 716/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk716<F: Float>(t1670: F, t4787: F, t10690: F, t591: F, t10552: F, t4790: F, t10681: F, t1685: F, t10696: F, t10699: F, t1965: F, t5365: F, t1961: F, t5397: F, t10773: F, t12066: F, t12070: F, t12073: F, t12077: F, t12081: F, t12084: F, t12087: F, t12090: F, t1975: F, t1979: F, t1980: F, t5368: F, t5373: F, t5393: F, t5398: F, t5401: F, t5405: F, t5408: F, t5409: F, t5412: F, t5415: F, t5416: F) -> (F,) {
    let t12095 = t1670 * t4787;
    let t12098 = t591 * t10690;
    let t12099 = t10552 * t4790;
    let t12102 = t10681 * t1685;
    let t12105 = t591 * t10696;
    let t12106 = t10552 * t10699;
    let t12109 = t5365 * t1965;
    let t12114 = t1961 * t5397;
    let t12117 = -6.0 * t5373 * t12066 + 0.96494049533612093922e2 * t5398 * t12070 - 0.35089340384731224426e1 * t5408 * t12073 + 0.51947267698127589897e2 * t5415 * t12077 - 0.19751789702565206229e-1 * t10773 + 6.0 * t5398 * t12081 - 0.35089340384731224426e1 * t12084 * t5409 + 0.35089340384731224426e1 * t5415 * t12087 + 0.17544670192365612213e1 * t12090 * t1980 + 0.17544670192365612213e1 * t5405 * t5412 + 0.51947267698127589899e2 * t12095 * t5416 - 0.1038945353962551798e3 * t12098 * t12099 + 0.58482233974552040708e0 * t1979 * t12102 + 0.1025389702100779493e4 * t12105 * t12106 + 3.0 * t12109 * t1975 + 3.0 * t5368 * t5393 + 0.96494049533612093922e2 * t12114 * t5401;
    (t12117,)
}
