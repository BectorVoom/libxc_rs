//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 677/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk677<F: Float>(t1056: F, t4621: F, t159: F, t23: F, t6: F, t107: F) -> (F, F, F, F) {
    let t4859 = t1056 * t4621;
    let t4863 = F::cast_from(1.0_f64) / t23 / t159;
    let t4864 = t6 * t4863;
    let t4865 = t107 * t4864;
    (t4859, t4863, t4864, t4865)
}
