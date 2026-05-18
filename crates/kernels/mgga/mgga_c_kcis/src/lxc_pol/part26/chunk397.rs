//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 397/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk397<F: Float>(t20: F, t2314: F, t92: F, t2: F, t647: F, t725: F, t2318: F, t2321: F, t2323: F, t15: F, t2317: F, t2320: F, t650: F, t720: F) -> (F, F, F, F) {
    let t2444 = t2314 * t92 * t20;
    let t2448 = t647 * t725 * t2;
    let t2456 = -F::new(0.44044444444444444445e-2) * t2318 + F::new(0.88088888888888888889e-2) * t2321 + F::new(0.55033333333333333333e-2) * t2323;
    let t2459 = -t2444 * t2317 / F::new(18.0) - t2448 * t650 / F::new(6.0) + t720 * t2320 / F::new(9.0) + t15 * t2456 / F::new(2.0);
    (t2444, t2448, t2456, t2459)
}
