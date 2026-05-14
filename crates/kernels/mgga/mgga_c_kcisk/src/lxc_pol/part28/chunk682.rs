//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 682/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk682<F: Float>(t227: F, t229: F, t3289: F, t7715: F, t7718: F, t44: F, t7714: F, t2452: F, t565: F, t2063: F, t2527: F, t5185: F, t5184: F, t5182: F, t2441: F, t5193: F, t5192: F, sigma2: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t228 = t227 <= zeta_threshold;
    let t7722 = piecewise3(t228, 0.0, 4.0 / 9.0 * t3289 * t7715 + 4.0 / 3.0 * t229 * t7718);
    let t7724 = (t7714 + t7722) * t44;
    let t8463 = 1.0 / t2452;
    let t8464 = sigma2 * t8463;
    let t8471 = piecewise3(t228, 0.0, t7718);
    let t8472 = t565 * t8471;
    let t8479 = t2063 * t2527;
    let t8480 = t5185 * t8479;
    let t8481 = t5184 * t8480;
    let t8482 = t5182 * t8481;
    let t8484 = t2063 * t2441;
    let t8485 = t5193 * t8484;
    let t8486 = t5192 * t8485;
    (t7724, t8463, t8464, t8471, t8472, t8480, t8481, t8482, t8485, t8486)
}
