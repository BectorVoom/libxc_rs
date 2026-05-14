//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1126/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1126<F: Float>(t1380: F, t16856: F, t337: F, t493: F, t2002: F, t5483: F, t1444: F, t6791: F, t9921: F, t2466: F, t3223: F, t161: F, t489: F, t6905: F, t10711: F, t10714: F, t16840: F, t16841: F, t16843: F, t16847: F, t16849: F, t16852: F, t16855: F) -> (F, F, F, F, F, F, F) {
    let t16860 = 2.0 / 45.0 * t493 * t1380 * t16856 * t337;
    let t16862 = 4.0 / 45.0 * t2002 * t5483;
    let t16864 = 4.0 / 45.0 * t1444 * t6791;
    let t16865 = 4.0 / 405.0 * t9921;
    let t16866 = t3223 * t2466;
    let t16867 = 2.0 / 405.0 * t16866;
    let t16869 = t161 * t489 * t6905;
    let t16870 = 2.0 / 45.0 * t16869;
    let t16871 = -t16840 - t16841 + t10711 + t10714 - t16843 + t16847 + t16849 + t16852 - t16855 - t16860 - t16862 - t16864 + t16865 - t16867 - t16870;
    (t16860, t16862, t16864, t16865, t16867, t16870, t16871)
}
