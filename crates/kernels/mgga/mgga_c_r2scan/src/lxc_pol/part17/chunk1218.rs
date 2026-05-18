//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1218/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1218<F: Float>(t39403: F, t41352: F, t41353: F, t41354: F, t41367: F, t41369: F, t41372: F, t43009: F, t43012: F, t43015: F, t43018: F, t43021: F) -> F {
    let t44188 = -t41352 - t41353 - F::new(0.21951497276451705328e0) * t43009 - t41354 - F::new(0.17336443480108537126e0) * t43012 + F::new(0.5200933044032561138e0) * t43015 + F::new(0.17336443480108537126e0) * t43018 - F::new(0.65854491829355115984e0) * t43021 + t41367 - t41369 - F::new(0.92461031893912198008e0) * t39403 + t41372;
    t44188
}
