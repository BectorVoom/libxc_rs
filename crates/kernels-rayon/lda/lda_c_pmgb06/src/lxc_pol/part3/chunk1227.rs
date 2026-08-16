//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1227/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1227(t1347: f64, t1795: f64, t118: f64, t5575: f64, t2174: f64, t415: f64, t14239: f64, t5522: f64, t10873: f64, t10876: f64, t10877: f64, t10881: f64, t10886: f64, t11710: f64, t14527: f64, t14529: f64, t14533: f64, t14536: f64, t14539: f64) -> f64 {
    let t14541 = t1795 * t1347;
    let t14543 = t5575 * t118;
    let t14544 = 0.1890324433388467_f64 * t14543;
    let t14545 = t2174 * t415;
    let t14547 = t14239 * t118;
    let t14549 = t5522 * t415;
    let t14550 = 0.1890324433388467_f64 * t14549;
    let t14551 = -0.031505407223141116_f64 * t10873 + t10876 + 0.031505407223141116_f64 * t10877 + 0.008980675507690957_f64 * t10881 - t10886 + 0.0878110494085338_f64 * t14527 - 0.031505407223141116_f64 * t14529 - 0.031505407223141116_f64 * t11710 * t118 - 0.09451622166942335_f64 * t14533 - t14536 + 0.02694202652307287_f64 * t14539 + 0.09451622166942335_f64 * t14541 - t14544 - 0.1890324433388467_f64 * t14545 + 0.09451622166942335_f64 * t14547 + t14550;
    t14551
}
