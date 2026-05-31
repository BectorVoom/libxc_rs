//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 334/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk334<F: Float>(t1684: F, t274: F, t1664: F, t1671: F, t1674: F, t1677: F, t964: F, t967: F) -> (F, F) {
    let t1685 = t1684 * t274;
    let t1692 = F::cast_from(0.258925e1_f64) * t1671 - t964 - F::cast_from(0.301925e0_f64) * t1664 + F::cast_from(0.16504875e0_f64) * t1674 - t967 - F::cast_from(0.82785e-1_f64) * t1677;
    (t1685, t1692)
}
