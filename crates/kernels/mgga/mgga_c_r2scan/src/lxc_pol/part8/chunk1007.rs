//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1007/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1007<F: Float>(t51: F, t476: F, t9874: F, t2517: F, t3010: F, t9870: F, t9868: F, zeta_threshold: F) -> (F, F) {
    let t52 = t51 <= zeta_threshold;
    let t9875 = t476 * t9874;
    let t9878 = piecewise3(t52, 0.0, 8.0 / 27.0 * t9870 - 2.0 / 3.0 * t2517 * t3010 + 2.0 / 3.0 * t9875);
    let t9880 = t9868 / 2.0 + t9878 / 2.0;
    (t9875, t9880)
}
