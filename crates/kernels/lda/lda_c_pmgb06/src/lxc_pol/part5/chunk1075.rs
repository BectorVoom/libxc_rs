//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1075/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1075<F: Float>(t132: F, t137: F, t19820: F, t19862: F, t19897: F, t19929: F, t465: F, t1423: F, t7696: F, t486: F, t7748: F, t6449: F, t831: F) -> (F, F, F, F) {
    let t19935 = t132 * t137 * t465 * (t19820 + t19862 + t19897 + t19929) / F::new(30.0);
    let t19936 = t1423 * t7696;
    let t19937 = F::new(2.0) / F::new(15.0) * t19936;
    let t19939 = t486 * t7748 / F::new(10.0);
    let t19941 = t831 * t6449 / F::new(5.0);
    (t19935, t19937, t19939, t19941)
}
