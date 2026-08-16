//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1210/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1210<F: Float>(t19763: F, t3438: F, t3437: F, t20157: F, t20160: F, t20162: F, t20165: F, t20167: F, t20170: F, t20174: F, t20176: F, t20179: F, t20181: F, t20183: F, t20186: F, t20188: F, t20192: F, t20195: F, t20198: F, t20201: F, t20203: F) -> (F, F) {
    let t20205 = t3438 * t19763;
    let t20206 = t3437 * t20205;
    let t20208 = -t20157 / F::cast_from(16.0_f64) + t20160 / F::cast_from(4.0_f64) + t20162 / F::cast_from(96.0_f64) + t20165 / F::cast_from(6.0_f64) + t20167 / F::cast_from(8.0_f64) + t20170 / F::cast_from(288.0_f64) + t20174 / F::cast_from(256.0_f64) - t20176 / F::cast_from(192.0_f64) - t20179 / F::cast_from(24.0_f64) + t20181 / F::cast_from(24.0_f64) - t20183 / F::cast_from(8.0_f64) + t20186 / F::cast_from(27.0_f64) - t20188 / F::cast_from(192.0_f64) - t20192 / F::cast_from(192.0_f64) - t20195 / F::cast_from(48.0_f64) + t20198 / F::cast_from(576.0_f64) + t20201 / F::cast_from(192.0_f64) + t20203 / F::cast_from(18.0_f64) + t20206 / F::cast_from(192.0_f64);
    (t20206, t20208)
}
