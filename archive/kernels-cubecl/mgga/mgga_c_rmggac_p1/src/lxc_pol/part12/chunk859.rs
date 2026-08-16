//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 859/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk859<F: Float>(t16156: F, t9096: F, t1356: F, t34822: F, t34826: F, t38944: F, t38946: F, t38948: F, t38958: F, t38963: F, t38965: F, t38969: F, t38971: F, t38974: F, t38976: F, t38978: F, t38981: F, t38984: F, t5752: F, t687: F) -> F {
    let t38986 = t16156 * t9096;
    let t38988 = F::cast_from(0.10215503974391481455e-3_f64) * t38944 + F::cast_from(0.72732431077987577943e-1_f64) * t38946 + F::cast_from(0.39914139006212695214e-1_f64) * t1356 * t38948 + F::cast_from(0.72732431077987577944e-1_f64) * t34822 + F::cast_from(0.36366215538993788972e-1_f64) * t34826 - F::cast_from(0.19957069503106347607e-1_f64) * t5752 * t687 + F::cast_from(0.42564599893297839398e-5_f64) * t38958 + F::cast_from(0.85129199786595678796e-5_f64) * t38963 - F::cast_from(0.33105799917009430643e-4_f64) * t38965 + t38969 + F::cast_from(0.25538759935978703638e-4_f64) * t38971 - F::cast_from(0.27274661654245341728e-1_f64) * t38974 + t38976 - F::cast_from(0.17961362552795712846e0_f64) * t38978 - F::cast_from(0.11974241701863808564e0_f64) * t38981 + F::cast_from(0.17961362552795712846e0_f64) * t38984 - F::cast_from(0.59590439850616975157e-4_f64) * t38986;
    t38988
}
