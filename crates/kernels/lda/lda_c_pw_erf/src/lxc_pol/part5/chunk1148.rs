//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1148/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1148<F: Float>(t22908: F, t22910: F, t22915: F, t22918: F, t22921: F, t22926: F, t22931: F, t22934: F, t22937: F, t22940: F, t22944: F, t22945: F, t22950: F, t14096: F, t14099: F, t14100: F, t14103: F, t14105: F, t22954: F, t22958: F, t22961: F, t22967: F, t22971: F, t22975: F, t22978: F, t22981: F) -> (F, F) {
    let t23326 = t22908 + t22910 - t22915 - t22918 + t22921 + t22926 - t22931 - t22934 + t22937 + t22940 - t22944 + t22945 - t22950;
    let t23330 = t22954 - t22958 - t22961 + t22967 + t22971 - t22975 + t22978 + t22981 + t14096 + t14099 + 0.9738937226128359 * t14100 + 0.10063568466999305 * t14103 - 0.03354522822333102 * t14105;
    (t23326, t23330)
}
