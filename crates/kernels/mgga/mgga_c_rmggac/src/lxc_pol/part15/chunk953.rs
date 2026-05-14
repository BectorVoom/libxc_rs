//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 953/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk953<F: Float>(t30453: F, t3351: F, t3352: F, t515: F, t8571: F, t8587: F, t26291: F, t29838: F, t34813: F, t36984: F, t42132: F, t42145: F, t42152: F, t46462: F, t46465: F, t46468: F, t47931: F, t47933: F, t47935: F, t47946: F, t47948: F, t47952: F, t47957: F, t5144: F, t739: F, t8800: F) -> (F,) {
    let t47961 = t3351 * t3352 * t515 * t30453;
    let t47963 = t8571 * t8587;
    let t47965 = 0.23948483403727617128e0 * t739 * t8800 * t5144 - 0.17961362552795712846e0 * t47931 - 0.17961362552795712846e0 * t47933 - 0.11974241701863808564e0 * t47935 - 0.71845450211182851384e0 * t26291 * t46462 + 0.95793933614910468512e0 * t29838 * t46465 + 0.71845450211182851384e0 * t34813 * t46468 + 0.72732431077987577943e-1 * t42132 - t42145 - t42152 - 0.25538759935978703638e-4 * t47946 - 0.25538759935978703638e-4 * t47948 + 0.25538759935978703638e-4 * t47952 - t36984 + 0.42564599893297839398e-5 * t47957 - 0.12769379967989351819e-4 * t47961 + 0.25538759935978703638e-4 * t47963;
    (t47965,)
}
