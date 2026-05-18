//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 298/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk298<F: Float>(t1025: F, t1043: F, t1041: F, t109: F, t675: F, t273: F, t978: F, t682: F, t964: F, t957: F, t963: F, t967: F) -> (F, F, F, F, F, F, F, F) {
    let t1044 = t1025 * t1043;
    let t1046 = F::new(16.081979498692537) * t1041 * t1044;
    let t1050 = t109 * t675;
    let t1054 = t273 * t978;
    let t1055 = t964 * t682;
    let t1058 = t957 * t682;
    let t1061 = t273 * t963;
    let t1062 = t964 * t967;
    (t1044, t1046, t1050, t1054, t1055, t1058, t1061, t1062)
}
