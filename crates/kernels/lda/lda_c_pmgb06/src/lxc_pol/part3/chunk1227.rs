//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1227/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1227<F: Float>(t1347: F, t1795: F, t118: F, t5575: F, t2174: F, t415: F, t14239: F, t5522: F, t10873: F, t10876: F, t10877: F, t10881: F, t10886: F, t11710: F, t14527: F, t14529: F, t14533: F, t14536: F, t14539: F) -> F {
    let t14541 = t1795 * t1347;
    let t14543 = t5575 * t118;
    let t14544 = F::cast_from(0.1890324433388467_f64) * t14543;
    let t14545 = t2174 * t415;
    let t14547 = t14239 * t118;
    let t14549 = t5522 * t415;
    let t14550 = F::cast_from(0.1890324433388467_f64) * t14549;
    let t14551 = -F::cast_from(0.031505407223141116_f64) * t10873 + t10876 + F::cast_from(0.031505407223141116_f64) * t10877 + F::cast_from(0.008980675507690957_f64) * t10881 - t10886 + F::cast_from(0.0878110494085338_f64) * t14527 - F::cast_from(0.031505407223141116_f64) * t14529 - F::cast_from(0.031505407223141116_f64) * t11710 * t118 - F::cast_from(0.09451622166942335_f64) * t14533 - t14536 + F::cast_from(0.02694202652307287_f64) * t14539 + F::cast_from(0.09451622166942335_f64) * t14541 - t14544 - F::cast_from(0.1890324433388467_f64) * t14545 + F::cast_from(0.09451622166942335_f64) * t14547 + t14550;
    t14551
}
