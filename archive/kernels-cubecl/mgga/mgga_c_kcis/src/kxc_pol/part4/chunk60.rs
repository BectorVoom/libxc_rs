//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 60/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk60<F: Float>(t122: F, t144: F, t145: F, t148: F, t85: F, t137: F, t113: F, t62: F) -> (F, F, F) {
    let t152 = F::cast_from(0.619125e-2_f64) * t144 * t145 - F::cast_from(0.79593333333333333331e-1_f64) * t85 * t148 * t122;
    let t153 = t152 * t137;
    let t154 = t62 * t113;
    (t152, t153, t154)
}
