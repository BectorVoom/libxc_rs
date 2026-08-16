//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 502/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk502<F: Float>(t1763: F, t4616: F, t352: F, t27: F, t29: F, t5840: F, t3908: F) -> (F, F) {
    let t6362 = t4616 * t1763;
    let t6363 = t6362 * t352;
    let t6374 = t5840 * t29 * t27;
    let t6376 = F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t6374 - t3908;
    (t6363, t6376)
}
