//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 332/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk332<F: Float>(t44: F, t1214: F, t1219: F, t472: F, t54: F, t419: F, zeta_threshold: F) -> (F, F, F) {
    let t45 = t44 <= zeta_threshold;
    let t1223 = piecewise3::<F>(t45, F::new(0.0), -F::new(2.0) / F::new(9.0) * t1214 + F::new(2.0) / F::new(3.0) * t472 * t1219);
    let t1224 = F::new(1.0) / t54;
    let t1225 = t419 * t419;
    (t1223, t1224, t1225)
}
