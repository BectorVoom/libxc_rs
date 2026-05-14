//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 361/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk361<F: Float>(t44: F, t1214: F, t1219: F, t472: F, t54: F, zeta_threshold: F) -> (F, F) {
    let t45 = t44 <= zeta_threshold;
    let t1223 = piecewise3(t45, 0.0, -2.0 / 9.0 * t1214 + 2.0 / 3.0 * t472 * t1219);
    let t1224 = 1.0 / t54;
    (t1223, t1224)
}
