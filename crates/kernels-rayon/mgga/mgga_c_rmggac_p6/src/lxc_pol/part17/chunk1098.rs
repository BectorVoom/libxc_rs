//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1098/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1098(t1756: f64, t3351: f64, t498: f64, t515: f64, t7231: f64, t30453: f64, t3352: f64, t8571: f64, t8587: f64, t26291: f64, t29838: f64, t34813: f64, t36984: f64, t42132: f64, t42145: f64, t42152: f64, t46462: f64, t46465: f64, t46468: f64, t47931: f64, t47933: f64, t47935: f64, t47946: f64, t47948: f64, t47952: f64, t5144: f64, t739: f64, t8800: f64) -> f64 {
    let t47957 = t3351 * t7231 * t515 * t1756 * t498;
    let t47961 = t3351 * t3352 * t515 * t30453;
    let t47963 = t8571 * t8587;
    let t47965 = 0.23948483403727617128e0_f64 * t739 * t8800 * t5144 - 0.17961362552795712846e0_f64 * t47931 - 0.17961362552795712846e0_f64 * t47933 - 0.11974241701863808564e0_f64 * t47935 - 0.71845450211182851384e0_f64 * t26291 * t46462 + 0.95793933614910468512e0_f64 * t29838 * t46465 + 0.71845450211182851384e0_f64 * t34813 * t46468 + 0.72732431077987577943e-1_f64 * t42132 - t42145 - t42152 - 0.25538759935978703638e-4_f64 * t47946 - 0.25538759935978703638e-4_f64 * t47948 + 0.25538759935978703638e-4_f64 * t47952 - t36984 + 0.42564599893297839398e-5_f64 * t47957 - 0.12769379967989351819e-4_f64 * t47961 + 0.25538759935978703638e-4_f64 * t47963;
    t47965
}
