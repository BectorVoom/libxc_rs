//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 470/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk470<F: Float>(t60: F, t1805: F, t921: F, t5860: F, t1403: F, t284: F, t5865: F, t62: F, t815: F, t5864: F, t277: F, t352: F, t570: F, zeta_threshold: F) -> (F, F, F, F) {
    let t61 = t60 <= zeta_threshold;
    let t5870 = t921 * t1805;
    let t5873 = -t5860;
    let t5877 = piecewise3::<F>(t61, F::new(0.0), -F::new(8.0) / F::new(27.0) * t5865 * t284 - F::new(16.0) / F::new(9.0) * t1403 * t815 + F::new(4.0) / F::new(9.0) * t5870 * t284 + F::new(4.0) / F::new(3.0) * t62 * t5873);
    let t5878 = t5864 + t5877;
    let t5879 = t277 * t5878;
    let t5888 = t570 * t352;
    (t5873, t5878, t5879, t5888)
}
