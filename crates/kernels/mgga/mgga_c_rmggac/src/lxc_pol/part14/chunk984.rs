//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 984/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk984<F: Float>(t128: F, t30526: F, t8645: F, t338: F, t6444: F, t8649: F, t39665: F, t5259: F, t2392: F, t839: F, t25877: F, t40687: F, t793: F) -> (F, F, F, F, F, F) {
    let t40823 = t30526 * t128;
    let t40824 = t40823 * t8645;
    let t40826 = t6444 * t338;
    let t40827 = t40826 * t8649;
    let t40831 = t5259 * t39665;
    let t40832 = F::new(0.15965655602485078085e0) * t40831;
    let t40833 = t2392 * t839;
    let t40834 = t25877 * t40833;
    let t40842 = t793 * t40687;
    (t40824, t40827, t40832, t40833, t40834, t40842)
}
