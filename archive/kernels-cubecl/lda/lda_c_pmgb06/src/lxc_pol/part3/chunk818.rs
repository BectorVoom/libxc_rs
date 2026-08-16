//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 818/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk818<F: Float>(t1799: F, t415: F, t1347: F, t795: F, t118: F, t5522: F, t1795: F, t117: F, t123: F, t125: F, t2777: F, t2814: F, t3474: F, t3478: F, t3481: F, t5543: F, t5610: F, t5615: F, t5620: F, t5622: F, t5625: F, t5627: F, t5689: F) -> F {
    let t5697 = F::cast_from(0.06301081444628223_f64) * t1799 * t415;
    let t5698 = t795 * t1347;
    let t5701 = F::cast_from(0.06301081444628223_f64) * t5522 * t118;
    let t5702 = t1795 * t415;
    let t5705 = t5610 - F::cast_from(0.04789693604101844_f64) * t3474 + F::cast_from(0.008980675507690957_f64) * t3478 + F::cast_from(0.006584630109636494_f64) * t5615 - t5620 - F::cast_from(0.003950778065781896_f64) * t5622 - F::cast_from(0.0004954275694490498_f64) * t5625 - F::cast_from(0.06301081444628223_f64) * t5627 - F::cast_from(0.005388405304614574_f64) * t123 * t125 * t5689 * t117 - F::cast_from(0.031505407223141116_f64) * t5543 * t118 - t5697 - F::cast_from(0.031505407223141116_f64) * t5698 + t5701 + F::cast_from(0.06301081444628223_f64) * t5702 + t2777 + t3481 + F::cast_from(0.031505407223141116_f64) * t2814;
    t5705
}
