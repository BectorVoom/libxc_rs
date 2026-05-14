//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 908/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk908<F: Float>(t11886: F, t11502: F, t11506: F, t11554: F, t986: F, t3276: F, t3275: F, t11540: F, t3579: F, t11496: F, t3263: F, t3262: F, t3574: F, t983: F, t8601: F, t9573: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12238 = 4.0 / 3.0 * t11886;
    let t12381 = t11506 * t11502;
    let t12382 = 3.0 / 2.0 * t12381;
    let t12383 = t11554 * t986;
    let t12384 = t3276 * t12383;
    let t12385 = t3275 * t12384;
    let t12386 = 5.0 / 8.0 * t12385;
    let t12387 = t3579 * t11540;
    let t12388 = t12387 / 2.0;
    let t12391 = t11496 * t986;
    let t12392 = t3263 * t12391;
    let t12393 = t3262 * t12392;
    let t12394 = 3.0 / 2.0 * t12393;
    let t12395 = t3574 * t983;
    let t12396 = t3276 * t12395;
    let t12397 = t3262 * t12396;
    let t12398 = 15.0 / 8.0 * t12397;
    let t12405 = t3275 * t3263 * t8601;
    let t12406 = t12405 / 4.0;
    let t12409 = t3275 * t3263 * t9573;
    (t12238, t12382, t12383, t12384, t12386, t12388, t12391, t12392, t12394, t12395, t12396, t12398, t12406, t12409)
}
