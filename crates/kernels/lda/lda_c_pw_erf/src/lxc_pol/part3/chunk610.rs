//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 610/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk610<F: Float>(t11: F, t3501: F, t3412: F, t503: F, t25: F, t3472: F, t3473: F, t3478: F, t3483: F, t3487: F, t3490: F, t3493: F, t3496: F, t3499: F) -> (F, F, F, F) {
    let t3502 = t11 * t3501;
    let t3504 = t503 * t3412;
    let t3505 = t11 * t3504;
    let t3507 = -t3472 - F::cast_from(0.02666666666666667_f64) * t3473 + F::cast_from(0.013333333333333334_f64) * t25 * t3478 - F::cast_from(0.006666666666666667_f64) * t25 * t3483 - F::new(0.04) * t25 * t3487 + F::new(0.04) * t25 * t3490 - F::cast_from(0.07198333333333333_f64) * t3493 + F::cast_from(0.14396666666666666_f64) * t3496 - F::cast_from(0.07198333333333333_f64) * t3499 - F::new(0.21595) * t3502 + F::new(0.21595) * t3505;
    (t3502, t3504, t3505, t3507)
}
