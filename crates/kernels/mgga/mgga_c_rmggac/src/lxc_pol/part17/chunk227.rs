//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 227/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk227<F: Float>(t446: F, t500: F, t385: F, t422: F, t388: F, t421: F, t155: F, t389: F, t409: F) -> (F, F, F, F, F, F, F) {
    let t1023 = t500 * t446;
    let t1027 = F::new(8.0) * t385 * t422;
    let t1028 = t388 * t421;
    let t1029 = t155 * t1028;
    let t1031 = t385 * t389;
    let t1037 = t409 * t409;
    let t1038 = F::new(1.0) / t1037;
    (t1023, t1027, t1028, t1029, t1031, t1037, t1038)
}
