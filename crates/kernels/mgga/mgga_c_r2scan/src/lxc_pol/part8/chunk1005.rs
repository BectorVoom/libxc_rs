//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1005/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1005<F: Float>(t44: F, t472: F, t9864: F, t2509: F, t3002: F, t9859: F, t3007: F, t893: F, zeta_threshold: F) -> (F, F, F) {
    let t45 = t44 <= zeta_threshold;
    let t9865 = t472 * t9864;
    let t9868 = piecewise3(t45, 0.0, 8.0 / 27.0 * t9859 - 2.0 / 3.0 * t2509 * t3002 + 2.0 / 3.0 * t9865);
    let t9869 = t3007 * t893;
    (t9865, t9868, t9869)
}
