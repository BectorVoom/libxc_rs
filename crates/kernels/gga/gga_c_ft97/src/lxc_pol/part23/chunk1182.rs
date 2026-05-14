//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1182/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1182<F: Float>(t29265: F, t681: F, t89: F, t2399: F, t7093: F, t28842: F, t870: F, t7055: F, t8232: F, t29131: F, t8392: F, t29212: F, t56110: F, t6360: F, t29068: F, t46862: F) -> (F, F, F, F, F, F, F, F) {
    let t114747 = 2.0 / 9.0 * t89 * t681 * t29265;
    let t114749 = t89 * t2399 * t7093;
    let t114751 = t28842 * t870;
    let t114757 = t8232 * t7055;
    let t114770 = 4.0 / 3.0 * t8392 * t29131;
    let t114772 = 4.0 / 27.0 * t8392 * t29212;
    let t114792 = t56110 * t6360;
    let t114818 = t46862 * t29068;
    (t114747, t114749, t114751, t114757, t114770, t114772, t114792, t114818)
}
