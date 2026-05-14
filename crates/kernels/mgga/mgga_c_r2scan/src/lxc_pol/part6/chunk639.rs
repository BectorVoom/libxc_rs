//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 639/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk639<F: Float>(t44: F, t1216: F, t2509: F, t2512: F, t415: F, t1224: F, t893: F, zeta_threshold: F) -> (F, F) {
    let t45 = t44 <= zeta_threshold;
    let t2516 = piecewise3(t45, 0.0, -2.0 / 9.0 * t2509 * t415 + 4.0 / 3.0 * t2512 * t1216);
    let t2517 = t1224 * t893;
    (t2516, t2517)
}
