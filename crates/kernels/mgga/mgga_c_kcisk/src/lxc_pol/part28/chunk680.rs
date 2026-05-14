//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 680/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk680<F: Float>(t222: F, t2059: F, t167: F, t3281: F, t224: F, t3277: F, t2063: F, zeta_threshold: F) -> (F, F, F, F) {
    let t223 = t222 <= zeta_threshold;
    let t7706 = t2059 * t2059;
    let t7710 = 2.0 * t167 + 2.0 * t3281;
    let t7714 = piecewise3(t223, 0.0, 4.0 / 9.0 * t3277 * t7706 + 4.0 / 3.0 * t224 * t7710);
    let t7715 = t2063 * t2063;
    (t7706, t7710, t7714, t7715)
}
