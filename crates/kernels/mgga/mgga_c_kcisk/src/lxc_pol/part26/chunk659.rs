//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 659/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk659<F: Float>(t222: F, t227: F, t224: F, t3277: F, t7706: F, t7710: F, t2063: F, t229: F, t3289: F, t44: F, t295: F, t3532: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t223 = t222 <= zeta_threshold;
    let t228 = t227 <= zeta_threshold;
    let t7714 = piecewise3(t223, 0.0, 4.0 / 9.0 * t3277 * t7706 + 4.0 / 3.0 * t224 * t7710);
    let t7715 = t2063 * t2063;
    let t7718 = -t7710;
    let t7722 = piecewise3(t228, 0.0, 4.0 / 9.0 * t3289 * t7715 + 4.0 / 3.0 * t229 * t7718);
    let t7724 = (t7714 + t7722) * t44;
    let t7727 = piecewise3(t223, 0.0, t7710);
    let t7728 = t295 * t7727;
    let t7736 = t3532 * t7706;
    (t7715, t7718, t7724, t7727, t7728, t7736)
}
