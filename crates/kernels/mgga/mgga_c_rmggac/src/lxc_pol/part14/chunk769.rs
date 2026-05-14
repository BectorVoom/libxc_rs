//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 769/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk769<F: Float>(t2392: F, t4905: F, t26291: F, t16156: F, t9096: F, t1356: F, t34822: F, t34826: F, t38944: F, t38946: F, t38948: F, t38958: F, t38963: F, t38965: F, t38969: F, t38971: F, t38974: F, t38976: F, t38978: F, t38981: F, t5752: F, t687: F) -> (F, F) {
    let t38983 = t2392 * t4905;
    let t38984 = t26291 * t38983;
    let t38986 = t16156 * t9096;
    let t38988 = 0.10215503974391481455e-3 * t38944 + 0.72732431077987577943e-1 * t38946 + 0.39914139006212695214e-1 * t1356 * t38948 + 0.72732431077987577944e-1 * t34822 + 0.36366215538993788972e-1 * t34826 - 0.19957069503106347607e-1 * t5752 * t687 + 0.42564599893297839398e-5 * t38958 + 0.85129199786595678796e-5 * t38963 - 0.33105799917009430643e-4 * t38965 + t38969 + 0.25538759935978703638e-4 * t38971 - 0.27274661654245341728e-1 * t38974 + t38976 - 0.17961362552795712846e0 * t38978 - 0.11974241701863808564e0 * t38981 + 0.17961362552795712846e0 * t38984 - 0.59590439850616975157e-4 * t38986;
    (t38983, t38988)
}
