//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 937/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk937<F: Float>(t13087: F, t132: F, t2851: F, t823: F, t1995: F, t3223: F, t1981: F, t835: F, t1461: F, t1835: F, t1902: F, t3213: F) -> (F, F, F, F, F, F) {
    let t13088 = t13087 / F::cast_from(45.0_f64);
    let t13090 = t132 * t2851 * t823;
    let t13139 = t3223 * t1995;
    let t13140 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t13139;
    let t13177 = t1981 * t835;
    let t13182 = t1461 * t1835;
    let t13243 = t3213 * t1902;
    (t13088, t13090, t13140, t13177, t13182, t13243)
}
