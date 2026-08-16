//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 829/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk829<F: Float>(t2209: F, t342: F, t1227: F, t769: F, t1282: F, t34: F, t1234: F, t2247: F, t2248: F, t3505: F, t3517: F, t3525: F, t3644: F, t3646: F, t3654: F, t5820: F, t5821: F, t5825: F, t5826: F) -> (F, F, F, F, F) {
    let t5866 = t2209 * t342;
    let t5870 = t769 * t1227;
    let t5874 = t34 * t1282;
    let t5875 = t769 * t1234;
    let t5879 = -t3505 - F::cast_from(1.532671111111111_f64) * t3644 + F::cast_from(0.5747516666666667_f64) * t3646 - F::cast_from(1.724255_f64) * t3654 - t3517 + t3525 + t5820 + t5821 - t5825 - t5826 + F::cast_from(10.34553_f64) * t2247 * t2248 * t5866 + F::cast_from(5.172765_f64) * t2247 * t2248 * t5870 - F::cast_from(20.69106_f64) * t2247 * t5874 * t5875;
    (t5866, t5870, t5874, t5875, t5879)
}
