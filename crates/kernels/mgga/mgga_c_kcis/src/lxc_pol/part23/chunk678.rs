//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 678/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk678<F: Float>(t8189: F, t8192: F, t8194: F, t8197: F, t8199: F, t8201: F, t8203: F, t8205: F) -> (F,) {
    let t8251 = 0.9375e-1 * t8189 - 0.9375e-1 * t8192 - 0.25e0 * t8194 + 0.625e-1 * t8197 - 0.20234375e-1 * t8199 + 0.20234375e-1 * t8201 + 0.10791666666666666667e0 * t8203 - 0.26979166666666666667e-1 * t8205;
    (t8251,)
}
