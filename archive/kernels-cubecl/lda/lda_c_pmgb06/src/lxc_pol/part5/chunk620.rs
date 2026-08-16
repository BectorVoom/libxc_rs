//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 620/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk620<F: Float>(t132: F, t4810: F, t1517: F, t802: F, t1872: F, t464: F, t1547: F, t823: F, t1554: F, t852: F, t161: F, t1: F, t1414: F) -> (F, F, F, F, F, F, F, F) {
    let t4812 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t132 * t4810;
    let t4814 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t802 * t1517;
    let t4815 = t1872 * t464;
    let t4836 = t1547 * t823;
    let t4837 = t132 * t4836;
    let t4844 = t1554 * t852;
    let t4845 = t161 * t4844;
    let t4851 = t1414 * t1;
    (t4812, t4814, t4815, t4836, t4837, t4844, t4845, t4851)
}
