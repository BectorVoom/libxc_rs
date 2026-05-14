//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 799/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk799<F: Float>(t39171: F, t8571: F, t1970: F, t236: F, t498: F, t6172: F, t7231: F, t321: F, t3352: F, t34807: F, t34810: F, t38934: F, t45012: F, t45018: F, t45020: F, t45026: F, t45032: F, t45038: F, t45044: F, t45048: F, t45055: F, t45060: F, t45062: F) -> (F,) {
    let t45064 = t8571 * t39171;
    let t45069 = t1970 * t7231 * t236 * t6172 * t498;
    let t45074 = t1970 * t3352 * t236 * t6172 * t321;
    let t45076 = 0.25538759935978703638e-4 * t45012 + 0.25538759935978703638e-4 * t45018 + 0.85129199786595678796e-5 * t45020 + 0.85129199786595678796e-5 * t45026 + 0.85129199786595678796e-5 * t45032 - t34807 - 0.33335697577410973224e-1 * t34810 + 0.42564599893297839398e-5 * t45038 - 0.42564599893297839398e-5 * t45044 - 0.42564599893297839398e-5 * t45048 - 0.59590439850616975157e-4 * t38934 - 0.25538759935978703638e-4 * t45055 + 0.95770349759920138642e-4 * t45060 - 0.31923449919973379548e-4 * t45062 - 0.25538759935978703638e-4 * t45064 + 0.42564599893297839398e-5 * t45069 - 0.12769379967989351819e-4 * t45074;
    (t45076,)
}
