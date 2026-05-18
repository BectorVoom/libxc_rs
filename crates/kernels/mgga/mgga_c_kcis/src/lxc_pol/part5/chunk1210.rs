//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1210/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1210<F: Float>(t19763: F, t3438: F, t3437: F, t20157: F, t20160: F, t20162: F, t20165: F, t20167: F, t20170: F, t20174: F, t20176: F, t20179: F, t20181: F, t20183: F, t20186: F, t20188: F, t20192: F, t20195: F, t20198: F, t20201: F, t20203: F) -> (F, F) {
    let t20205 = t3438 * t19763;
    let t20206 = t3437 * t20205;
    let t20208 = -t20157 / F::new(16.0) + t20160 / F::new(4.0) + t20162 / F::new(96.0) + t20165 / F::new(6.0) + t20167 / F::new(8.0) + t20170 / F::new(288.0) + t20174 / F::new(256.0) - t20176 / F::new(192.0) - t20179 / F::new(24.0) + t20181 / F::new(24.0) - t20183 / F::new(8.0) + t20186 / F::new(27.0) - t20188 / F::new(192.0) - t20192 / F::new(192.0) - t20195 / F::new(48.0) + t20198 / F::new(576.0) + t20201 / F::new(192.0) + t20203 / F::new(18.0) + t20206 / F::new(192.0);
    (t20206, t20208)
}
