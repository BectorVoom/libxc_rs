//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 701/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk701<F: Float>(t530: F, t7399: F, t1990: F, t8571: F, t884: F, t8960: F, t8405: F, t8408: F, t8411: F, t8414: F, t8458: F, t8500: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9234 = t530 * t7399;
    let t9236 = t8571 * t1990;
    let t9238 = t884 * t8960;
    let t9268 = F::new(0.5987120850931904282e-1) * t8405;
    let t9269 = F::new(0.8980681276397856423e-1) * t8408;
    let t9270 = F::new(0.17961362552795712846e0) * t8411;
    let t9271 = F::new(0.5987120850931904282e-1) * t8414;
    let t9282 = F::new(0.1064114997332445985e-4) * t8458;
    let t9297 = F::new(0.19863479950205658386e-4) * t8500;
    (t9234, t9236, t9238, t9268, t9269, t9270, t9271, t9282, t9297)
}
