//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 823/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk823<F: Float>(t5788: F, t410: F, t776: F, t360: F, t2233: F, t365: F, t350: F, t1271: F, t780: F, t955: F, t2210: F, t348: F) -> (F, F, F, F, F, F, F, F) {
    let t5789 = F::cast_from(0.6495611111111111_f64) * t5788;
    let t5790 = t410 * t776;
    let t5791 = t360 * t5790;
    let t5793 = t365 * t2233;
    let t5795 = F::cast_from(1.46904_f64) * t5793 * t350;
    let t5796 = t1271 * t780;
    let t5797 = t5796 * t955;
    let t5799 = t348 * t2210;
    (t5789, t5790, t5791, t5793, t5795, t5796, t5797, t5799)
}
