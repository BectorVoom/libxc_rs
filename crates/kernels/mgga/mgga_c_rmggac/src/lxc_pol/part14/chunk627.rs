//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 627/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk627<F: Float>(t333: F, t8936: F, t118: F, t4616: F, t352: F, t305: F, t8821: F, t558: F, t664: F) -> (F, F, F, F, F) {
    let t8937 = t8936 * t333;
    let t8940 = t118 * t4616;
    let t8941 = t8936 * t352;
    let t8944 = t305 * t8821;
    let t8946 = t664 * t558;
    (t8937, t8940, t8941, t8944, t8946)
}
