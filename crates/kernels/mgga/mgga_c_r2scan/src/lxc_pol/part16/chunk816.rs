//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 816/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk816<F: Float>(t51: F, t1368: F, t3010: F, t8571: F, t1217: F, t2474: F, t419: F, t53: F, t8576: F, t8575: F, t60: F, zeta_threshold: F) -> (F, F, F) {
    let t52 = t51 <= zeta_threshold;
    let t8581 = t1368 * t3010;
    let t8584 = -t8571;
    let t8588 = piecewise3::<f64>(t52, F::new(0.0), -F::new(8.0) / F::new(27.0) * t8576 * t419 - F::new(16.0) / F::new(9.0) * t2474 * t1217 + F::new(4.0) / F::new(9.0) * t8581 * t419 + F::new(4.0) / F::new(3.0) * t53 * t8584);
    let t8589 = t8575 + t8588;
    let t8590 = t8589 * t60;
    (t8584, t8589, t8590)
}
