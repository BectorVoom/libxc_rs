//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 754/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk754<F: Float>(t2043: F, t432: F, t1395: F, t2064: F, t137: F, t132: F, t3058: F, t822: F, t1512: F, t824: F, t443: F, t472: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4973 = t432 * t2043 / F::cast_from(15.0_f64);
    let t4974 = t1395 * t2064;
    let t4975 = t137 * t4974;
    let t4977 = t132 * t4975 / F::cast_from(15.0_f64);
    let t4978 = t3058 * t822;
    let t4979 = t137 * t4978;
    let t4981 = t132 * t4979 / F::cast_from(30.0_f64);
    let t4983 = t1512 * t824 / F::cast_from(30.0_f64);
    let t4989 = t472 * t443;
    (t4973, t4974, t4975, t4977, t4978, t4979, t4981, t4983, t4989)
}
