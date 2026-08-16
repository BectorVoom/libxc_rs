//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 812/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk812<F: Float>(t415: F, t763: F, t5594: F, t1664: F, t1852: F, t10: F, t127: F, t1568: F, t3313: F, t3322: F, t426: F, t5588: F, t5591: F, t5596: F, t5598: F, t5599: F, t5603: F) -> (F, F, F, F) {
    let t5607 = t415 * t763;
    let t5609 = F::cast_from(1.9486833333333333_f64) * t5607 * t5594;
    let t5610 = t1852 * t1664;
    let t5614 = F::cast_from(5.87616_f64) * t127 * t1852 * t1568 + t5588 + t5591 - t5596 - t5598 + F::cast_from(3.0_f64) * t426 * t10 * t5599 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t426 * t10 * t5603 - t5609 - F::cast_from(6.0_f64) * t426 * t10 * t5610 + t3313 - t3322;
    (t5607, t5609, t5610, t5614)
}
