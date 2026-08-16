//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 859/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk859(t16156: f64, t9096: f64, t1356: f64, t34822: f64, t34826: f64, t38944: f64, t38946: f64, t38948: f64, t38958: f64, t38963: f64, t38965: f64, t38969: f64, t38971: f64, t38974: f64, t38976: f64, t38978: f64, t38981: f64, t38984: f64, t5752: f64, t687: f64) -> f64 {
    let t38986 = t16156 * t9096;
    let t38988 = 0.10215503974391481455e-3_f64 * t38944 + 0.72732431077987577943e-1_f64 * t38946 + 0.39914139006212695214e-1_f64 * t1356 * t38948 + 0.72732431077987577944e-1_f64 * t34822 + 0.36366215538993788972e-1_f64 * t34826 - 0.19957069503106347607e-1_f64 * t5752 * t687 + 0.42564599893297839398e-5_f64 * t38958 + 0.85129199786595678796e-5_f64 * t38963 - 0.33105799917009430643e-4_f64 * t38965 + t38969 + 0.25538759935978703638e-4_f64 * t38971 - 0.27274661654245341728e-1_f64 * t38974 + t38976 - 0.17961362552795712846e0_f64 * t38978 - 0.11974241701863808564e0_f64 * t38981 + 0.17961362552795712846e0_f64 * t38984 - 0.59590439850616975157e-4_f64 * t38986;
    t38988
}
