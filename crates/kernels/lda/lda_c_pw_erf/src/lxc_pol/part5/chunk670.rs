//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 670/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk670<F: Float>(t168: F, t2292: F, t635: F, t1905: F, t632: F, t1143: F, t781: F, t1896: F, t242: F, t5446: F, t1901: F, t1125: F, t153: F, t865: F) -> (F, F, F, F, F, F, F) {
    let t5887 = F::cast_from(0.039794582218349216_f64) * t168 * t635 * t2292;
    let t5891 = F::cast_from(0.1675256410710088_f64) * t1905 * t632;
    let t5892 = t781 * t1143;
    let t5894 = t1896 * t242;
    let t5897 = F::cast_from(0.1675256410710088_f64) * t5446 * t242;
    let t5898 = t1901 * t632;
    let t5904 = t153 * t1125 * t865;
    (t5887, t5891, t5892, t5894, t5897, t5898, t5904)
}
