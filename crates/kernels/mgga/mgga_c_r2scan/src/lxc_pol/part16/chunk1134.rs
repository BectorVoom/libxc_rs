//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1134/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1134<F: Float>(t10610: F, t3263: F, t42392: F, t11479: F, t3275: F, t7040: F, t14160: F, t3245: F, t3270: F, t3269: F, t2850: F, t6967: F) -> (F, F, F, F) {
    let t42395 = F::new(3.0) * t10610 * t3263 * t42392;
    let t42398 = t3275 * t11479 * t7040 / F::new(2.0);
    let t42399 = t14160 * t3245;
    let t42400 = t3270 * t42399;
    let t42402 = t3269 * t42400 / F::new(2.0);
    let t42403 = t6967 * t2850;
    (t42395, t42398, t42402, t42403)
}
