//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 568/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk568<F: Float>(t227: F, t8463: F, t807: F, t2356: F, t2361: F, t2671: F, t7718: F, t565: F, t806: F, t564: F, t2360: F, t2670: F, t2063: F, t2527: F, t5185: F, t5184: F, t5182: F, sigma2: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t228 = t227 <= zeta_threshold;
    let t8464 = sigma2 * t8463;
    let t8465 = t8464 * t807;
    let t8467 = t2356 * t2361;
    let t8469 = t2356 * t2671;
    let t8471 = piecewise3(t228, 0.0, t7718);
    let t8472 = t565 * t8471;
    let t8473 = t8472 * t806;
    let t8474 = t564 * t8473;
    let t8476 = t2360 * t2670;
    let t8477 = t564 * t8476;
    let t8479 = t2063 * t2527;
    let t8480 = t5185 * t8479;
    let t8481 = t5184 * t8480;
    let t8482 = t5182 * t8481;
    (t8464, t8465, t8467, t8469, t8472, t8473, t8474, t8476, t8477, t8480, t8481, t8482)
}
