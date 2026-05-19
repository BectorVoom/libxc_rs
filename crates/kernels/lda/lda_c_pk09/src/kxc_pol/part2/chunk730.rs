//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 730/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk730<F: Float>(t1819: F, t1947: F, t2042: F, t6319: F, t6325: F, t6464: F, t1: F, t2954: F, t2961: F, t2965: F, t2250: F, t633: F) -> (F, F, F, F, F, F, F) {
    let t7532 = t1819 * t1947;
    let t7533 = t7532 * t2042;
    let t7537 = F::new(1.5625) * t6319;
    let t7539 = F::cast_from(1.0416666666666667_f64) * t6325;
    let t7545 = F::cast_from(0.3472222222222222_f64) * t6464;
    let t7566 = t1 * t2954;
    let t7568 = t2961 - t2965;
    let t7577 = t2250 * t633;
    (t7533, t7537, t7539, t7545, t7566, t7568, t7577)
}
