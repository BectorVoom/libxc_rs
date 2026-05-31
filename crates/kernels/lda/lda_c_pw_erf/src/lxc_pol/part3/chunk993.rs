//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 993/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk993<F: Float>(t10823: F, t10849: F, t11557: F, t11561: F, t11563: F, t11568: F, t11570: F, t11574: F, t11577: F, t11588: F, t11597: F, t1550: F, t1733: F, t1881: F, t2211: F, t2764: F, t2767: F, t2799: F, t4117: F, t4441: F, t5670: F, t777: F, t9127: F) -> F {
    let t11599 = F::cast_from(0.19513566535229734_f64) * t11557 + F::cast_from(0.0001639671923854359_f64) * t11561 - F::cast_from(0.15965645347006147_f64) * t11563 + t11568 - F::cast_from(9.0_f64) * t2764 * t11570 - F::cast_from(18.0_f64) * t11574 * t2767 - F::cast_from(6.0_f64) * t2764 * t11577 + F::cast_from(6.0_f64) * t4117 * t4441 + F::cast_from(3.0_f64) * t2211 * t10823 + F::cast_from(9.0_f64) * t2211 * t10849 - F::cast_from(3.0_f64) * t1881 * t2799 + F::cast_from(3.0_f64) * t1733 * t11588 - F::cast_from(3.0_f64) * t777 * t9127 + F::cast_from(3.0_f64) * t5670 * t1550 - F::cast_from(0.9247854820715865_f64) * t11597;
    t11599
}
