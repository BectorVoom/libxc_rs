//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1084/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1084<F: Float>(t20920: F, t20922: F, t20925: F, t20929: F, t20931: F, t20934: F, t20937: F, t20940: F, t20943: F, t20946: F, t20950: F, t20953: F, t13949: F, t20955: F, t20958: F, t20961: F, t20964: F, t20967: F, t20969: F, t20971: F, t20973: F, t20977: F, t20981: F, t20984: F) -> (F, F) {
    let t22037 = t20920 - t20922 - t20925 - t20929 - t20931 - t20934 - t20937 + t20940 - t20943 - t20946 - t20950 - t20953;
    let t22039 = -t20955 + t13949 + t20958 - t20961 + t20964 - t20967 - t20969 - t20971 + t20973 + t20977 - t20981 - t20984;
    (t22037, t22039)
}
