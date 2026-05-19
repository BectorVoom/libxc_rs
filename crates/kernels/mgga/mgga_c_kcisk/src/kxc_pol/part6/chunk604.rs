//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 604/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk604<F: Float>(t227: F, t297: F, t8459: F, t294: F, t2452: F, t807: F, t2356: F, t2361: F, t2671: F, t7718: F, t565: F, t806: F, t564: F, sigma2: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F) {
    let t228 = t227 <= zeta_threshold;
    let t8460 = t297 * t8459;
    let t8461 = t294 * t8460;
    let t8463 = F::new(1.0) / t2452;
    let t8464 = sigma2 * t8463;
    let t8465 = t8464 * t807;
    let t8467 = t2356 * t2361;
    let t8469 = t2356 * t2671;
    let t8471 = piecewise3::<F>(t228, F::new(0.0), t7718);
    let t8472 = t565 * t8471;
    let t8473 = t8472 * t806;
    let t8474 = t564 * t8473;
    (t8461, t8464, t8465, t8467, t8469, t8472, t8473, t8474)
}
