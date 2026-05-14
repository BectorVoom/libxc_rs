//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 282/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk282<F: Float>(t44: F, t51: F, t41: F, t899: F, t86: F, t898: F, t472: F, t889: F, t476: F, t893: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t45 = t44 <= zeta_threshold;
    let t52 = t51 <= zeta_threshold;
    let t900 = t41 * t899;
    let t901 = t898 * t86;
    let t902 = 0.19751673498613801407e-1 * t901;
    let t903 = t472 * t889;
    let t905 = piecewise3(t45, 0.0, 2.0 / 3.0 * t903);
    let t906 = t476 * t893;
    let t908 = piecewise3(t52, 0.0, 2.0 / 3.0 * t906);
    let t910 = t905 / 2.0 + t908 / 2.0;
    (t900, t902, t903, t906, t910)
}
