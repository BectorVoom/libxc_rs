//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 994/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk994<F: Float>(t2029: F, t4119: F, t9311: F, t9313: F, t1629: F, t1966: F, t439: F, t5201: F, t224: F, t4753: F, t446: F, t1427: F, t5187: F) -> (F, F, F, F, F, F) {
    let t11813 = t2029 * t4119;
    let t11815 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t9311;
    let t11816 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t9313;
    let t11820 = t439 * t1966 * t5201 * t1629 / F::cast_from(5.0_f64);
    let t11821 = t4753 * t224;
    let t11823 = t11821 * t446 / F::cast_from(15.0_f64);
    let t11825 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t5187 * t1427;
    (t11813, t11815, t11816, t11820, t11823, t11825)
}
