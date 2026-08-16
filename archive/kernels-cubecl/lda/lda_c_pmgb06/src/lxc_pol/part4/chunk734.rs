//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 734/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk734<F: Float>(t1959: F, t607: F, t1710: F, t883: F, t1447: F, t1912: F, t1916: F, t1920: F, t1444: F, t1911: F, t2979: F, t493: F) -> (F, F, F, F, F, F, F, F) {
    let t4717 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t1959 * t607;
    let t4718 = t883 * t1710;
    let t4721 = F::cast_from(4.0_f64) / F::cast_from(135.0_f64) * t1447 * t1912;
    let t4723 = F::cast_from(8.0_f64) / F::cast_from(135.0_f64) * t1447 * t1916;
    let t4725 = F::cast_from(4.0_f64) / F::cast_from(81.0_f64) * t1447 * t1920;
    let t4727 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t1444 * t1912;
    let t4728 = t2979 * t1911;
    let t4730 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t493 * t4728;
    (t4717, t4718, t4721, t4723, t4725, t4727, t4728, t4730)
}
