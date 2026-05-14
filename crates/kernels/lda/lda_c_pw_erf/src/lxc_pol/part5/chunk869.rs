//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 869/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk869<F: Float>(t1351: F, t3975: F, t1518: F, t185: F, t2099: F, t4500: F, t784: F, t4521: F, t811: F, t34: F, t2070: F, t807: F, t834: F, t211: F, t548: F, t812: F) -> (F, F, F, F, F, F, F, F) {
    let t13966 = t3975 * t1351;
    let t14004 = t185 * t1518 * t2099;
    let t14005 = 4.0 / 45.0 * t14004;
    let t14014 = t4500 * t784;
    let t14030 = t4521 * t811;
    let t14034 = t3975 * t34;
    let t14043 = t2070 * t807;
    let t14044 = t185 * t14043;
    let t14048 = t2070 * t834;
    let t14049 = t211 * t14048;
    let t14052 = t548 * t2070 * t812;
    (t13966, t14005, t14014, t14030, t14034, t14044, t14049, t14052)
}
