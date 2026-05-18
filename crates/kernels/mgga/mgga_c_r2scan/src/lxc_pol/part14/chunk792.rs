//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 792/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk792<F: Float>(t44: F, t1361: F, t35: F, t1216: F, t415: F, t1213: F, t1219: F, t2466: F, t2469: F, t40: F, t48: F, t6976: F, t4948: F, t893: F, zeta_threshold: F) -> (F, F, F) {
    let t45 = t44 <= zeta_threshold;
    let t6979 = t1361 * t35;
    let t6980 = t1216 * t415;
    let t6990 = piecewise3::<f64>(t45, F::new(0.0), -F::new(8.0) / F::new(27.0) * t6976 * t1213 + F::new(16.0) / F::new(9.0) * t6979 * t6980 + F::new(4.0) / F::new(9.0) * t2466 * t1219 + F::new(8.0) / F::new(3.0) * t48 * t1216 - F::new(8.0) * t2469 * t40);
    let t6991 = t4948 * t893;
    (t6980, t6990, t6991)
}
