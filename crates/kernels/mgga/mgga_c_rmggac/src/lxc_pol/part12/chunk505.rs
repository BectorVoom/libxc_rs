//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 505/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk505<F: Float>(t53: F, t50: F, t983: F, t1375: F, t1378: F, t154: F, t437: F, t5328: F, t5498: F, t814: F, t913: F, t916: F, t4408: F, t525: F, zeta_threshold: F) -> (F, F) {
    let t54 = t53 <= zeta_threshold;
    let t5501 = t983 * t50;
    let t5511 = piecewise3::<f64>(t54, F::new(0.0), F::new(8.0) / F::new(27.0) * t5498 * t913 - F::new(8.0) / F::new(9.0) * t5501 * t5328 - F::new(2.0) / F::new(9.0) * t1375 * t916 + F::new(4.0) / F::new(3.0) * t437 * t814 - F::new(4.0) * t1378 * t154);
    let t5512 = t4408 * t525;
    (t5511, t5512)
}
