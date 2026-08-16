//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 639/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk639<F: Float>(t1652: F, t36: F, t2079: F, t262: F, t2024: F, t570: F, t664: F, t333: F, t118: F, t4616: F, t352: F, t305: F, t8821: F) -> (F, F, F, F, F, F, F) {
    let t8924 = t36 * t1652;
    let t8926 = t2079 * t262 * t8924;
    let t8933 = t2024 * t1652;
    let t8936 = t664 * t570;
    let t8937 = t8936 * t333;
    let t8940 = t118 * t4616;
    let t8941 = t8936 * t352;
    let t8944 = t305 * t8821;
    (t8926, t8933, t8936, t8937, t8940, t8941, t8944)
}
