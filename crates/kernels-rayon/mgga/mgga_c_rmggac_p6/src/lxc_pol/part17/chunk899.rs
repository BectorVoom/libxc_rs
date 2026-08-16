//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 899/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk899(t40658: f64, t9222: f64, t39171: f64, t8571: f64, t1970: f64, t236: f64, t498: f64, t6172: f64, t7231: f64, t321: f64, t3352: f64, t34807: f64, t34810: f64, t38934: f64, t45012: f64, t45018: f64, t45020: f64, t45026: f64, t45032: f64, t45038: f64, t45044: f64, t45048: f64, t45055: f64, t45060: f64) -> f64 {
    let t45062 = t9222 * t40658;
    let t45064 = t8571 * t39171;
    let t45069 = t1970 * t7231 * t236 * t6172 * t498;
    let t45074 = t1970 * t3352 * t236 * t6172 * t321;
    let t45076 = 0.25538759935978703638e-4_f64 * t45012 + 0.25538759935978703638e-4_f64 * t45018 + 0.85129199786595678796e-5_f64 * t45020 + 0.85129199786595678796e-5_f64 * t45026 + 0.85129199786595678796e-5_f64 * t45032 - t34807 - 0.33335697577410973224e-1_f64 * t34810 + 0.42564599893297839398e-5_f64 * t45038 - 0.42564599893297839398e-5_f64 * t45044 - 0.42564599893297839398e-5_f64 * t45048 - 0.59590439850616975157e-4_f64 * t38934 - 0.25538759935978703638e-4_f64 * t45055 + 0.95770349759920138642e-4_f64 * t45060 - 0.31923449919973379548e-4_f64 * t45062 - 0.25538759935978703638e-4_f64 * t45064 + 0.42564599893297839398e-5_f64 * t45069 - 0.12769379967989351819e-4_f64 * t45074;
    t45076
}
