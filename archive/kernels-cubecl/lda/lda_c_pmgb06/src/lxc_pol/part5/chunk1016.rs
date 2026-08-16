//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1016/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1016<F: Float>(t19130: F, t3: F, t415: F, t7874: F, t10792: F, t10795: F, t10802: F, t10806: F, t10808: F, t10811: F, t10813: F, t10817: F, t10825: F, t10828: F, t118: F, t14501: F, t15163: F, t15166: F, t19126: F) -> (F, F) {
    let t19131 = t3 * t19130;
    let t19134 = t7874 * t415;
    let t19140 = F::cast_from(0.031505407223141116_f64) * t19126 - F::cast_from(0.005926167098672845_f64) * t15163 - F::cast_from(0.01185233419734569_f64) * t15166 - F::cast_from(0.031505407223141116_f64) * t19131 * t118 - F::cast_from(0.031505407223141116_f64) * t19134 - t10792 - t14501 + F::cast_from(0.0034679929861433484_f64) * t10795 - F::cast_from(0.0014862827083471494_f64) * t10802 - t10806 - t10808 - t10811 - F::cast_from(0.005926167098672845_f64) * t10813 + t10817 - F::cast_from(0.025899545097903542_f64) * t10825 - t10828;
    (t19131, t19140)
}
