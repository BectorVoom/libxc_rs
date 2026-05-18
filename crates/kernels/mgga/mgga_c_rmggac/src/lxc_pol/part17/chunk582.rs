//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 582/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk582<F: Float>(t641: F, t7926: F, t2019: F, t2128: F, t275: F, t2017: F, t262: F, t2016: F) -> (F, F, F, F, F) {
    let t7927 = t7926 * t641;
    let t7928 = t2019 * t7927;
    let t7930 = t275 * t2128;
    let t7931 = F::new(2.0) * t7930;
    let t7932 = t2017 * t262;
    let t7933 = t2016 * t7932;
    (t7927, t7928, t7931, t7932, t7933)
}
