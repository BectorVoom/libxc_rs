//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 736/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk736<F: Float>(t4695: F, t4881: F, t4883: F, t4886: F, t4892: F, t4894: F, t4896: F, t4898: F, t4703: F, t4880: F, t4891: F, t4901: F, t4968: F, t2850: F, t797: F, t2266: F, t481: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t6943 = 2.0 * t4695;
    let t6946 = 12.0 * t4881;
    let t6947 = 40.0 * t4883;
    let t6948 = 80.0 * t4886;
    let t6949 = 4.0 * t4892;
    let t6950 = 4.0 * t4894;
    let t6951 = 32.0 * t4896;
    let t6952 = 24.0 * t4898;
    let t6953 = -t6943 - t4880 + t6946 - t6947 - t6948 + t4891 + t6949 + t6950 - t4703 + t6951 + t6952 - t4901;
    let t6954 = 0.21687162600603479684e-1 * t4968;
    let t6955 = t2850 * t797;
    let t6957 = t2266 * t6955 * t481;
    (t6943, t6946, t6947, t6948, t6949, t6950, t6951, t6952, t6953, t6954, t6957)
}
