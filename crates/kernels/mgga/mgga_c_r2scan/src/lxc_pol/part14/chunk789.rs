//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 789/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk789<F: Float>(t4695: F, t4881: F, t4883: F, t4886: F, t4892: F, t4894: F, t4896: F, t4898: F, t4703: F, t4880: F, t4891: F, t4901: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6943 = F::new(2.0) * t4695;
    let t6946 = F::new(12.0) * t4881;
    let t6947 = F::new(40.0) * t4883;
    let t6948 = F::new(80.0) * t4886;
    let t6949 = F::new(4.0) * t4892;
    let t6950 = F::new(4.0) * t4894;
    let t6951 = F::new(32.0) * t4896;
    let t6952 = F::new(24.0) * t4898;
    let t6953 = -t6943 - t4880 + t6946 - t6947 - t6948 + t4891 + t6949 + t6950 - t4703 + t6951 + t6952 - t4901;
    (t6943, t6946, t6947, t6948, t6949, t6950, t6951, t6952, t6953)
}
