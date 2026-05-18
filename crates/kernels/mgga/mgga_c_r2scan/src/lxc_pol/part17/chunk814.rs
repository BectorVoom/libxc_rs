//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 814/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk814<F: Float>(t44: F, t6959: F, t2999: F, t4938: F, t1361: F, t3002: F, t1216: F, t4911: F, t1217: F, t2466: F, t415: F, t48: F, t3007: F, t4948: F, zeta_threshold: F) -> (F, F, F, F) {
    let t45 = t44 <= zeta_threshold;
    let t8560 = F::new(0.21687162600603479684e-1) * t6959;
    let t8561 = t4938 * t2999;
    let t8566 = t1361 * t3002;
    let t8571 = -F::new(2.0) * t1216 - F::new(6.0) * t4911;
    let t8575 = piecewise3::<f64>(t45, F::new(0.0), -F::new(8.0) / F::new(27.0) * t8561 * t415 + F::new(16.0) / F::new(9.0) * t2466 * t1217 + F::new(4.0) / F::new(9.0) * t8566 * t415 + F::new(4.0) / F::new(3.0) * t48 * t8571);
    let t8576 = t4948 * t3007;
    (t8560, t8571, t8575, t8576)
}
