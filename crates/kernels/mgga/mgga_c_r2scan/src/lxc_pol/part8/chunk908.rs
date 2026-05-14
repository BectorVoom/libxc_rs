//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 908/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk908<F: Float>(t44: F, t1217: F, t2466: F, t415: F, t48: F, t8561: F, t8566: F, t8571: F, t3007: F, t4948: F, t1368: F, t3010: F, zeta_threshold: F) -> (F, F, F, F) {
    let t45 = t44 <= zeta_threshold;
    let t8575 = piecewise3(t45, 0.0, -8.0 / 27.0 * t8561 * t415 + 16.0 / 9.0 * t2466 * t1217 + 4.0 / 9.0 * t8566 * t415 + 4.0 / 3.0 * t48 * t8571);
    let t8576 = t4948 * t3007;
    let t8581 = t1368 * t3010;
    let t8584 = -t8571;
    (t8575, t8576, t8581, t8584)
}
