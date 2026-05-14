//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1043/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1043<F: Float>(t10277: F, t10286: F, t10292: F, t10295: F, t10303: F, t11: F, t5: F, t5193: F, t7637: F, t8879: F, t146: F, t147: F, t3090: F, t921: F, t5109: F, t279: F, t5108: F, t6266: F, t6310: F, t6324: F, t6333: F, t8026: F, t8125: F, t8130: F, t9320: F, t9323: F, t9335: F, t9371: F, t9374: F, t9378: F, t9382: F, t9388: F) -> (F, F, F, F, F) {
    let t10308 = -t5193 + 20.0 / 3.0 * t7637 - 5.0 * t8879 + 5.0 * t5 * t11 * t10277 - 45.0 * param_eta * (t10286 + t10292 + t10295 + t10303);
    let t10310 = t146 * t147 * t10308;
    let t10315 = t3090 * t921;
    let t10316 = t5109 * t10315;
    let t10326 = t6266 - 0.16463622957338778996e-1 * t9320 - 0.29272321618148349056e-1 * t9323 + 0.43341108700271342816e-1 * t10310 * t279 - 0.64025200389650807209e0 * t8026 + 0.20958572791407956061e0 * t9335 - 0.7801399566048841707e0 * t5108 * t10316 - t6310 + t6324 + t6333 + 0.29272321618148349056e-1 * t9371 - 0.32927245914677557992e-1 * t9374 - 0.14636160809074174528e-1 * t9378 + 0.1047928639570397803e0 * t9382 - 0.43371823197556470519e-3 * t8125 - 0.19043987679069580388e-1 * t8130 - 0.20803732176130244552e1 * t9388;
    (t10308, t10310, t10315, t10316, t10326)
}
