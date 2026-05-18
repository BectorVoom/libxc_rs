//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 987/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk987<F: Float>(t14903: F, t1131: F, t1187: F, t5931: F, t1138: F, t2817: F, t2820: F, t780: F, t168: F, t2782: F, t861: F, t1125: F, t153: F, t1891: F) -> (F, F, F, F, F) {
    let t14904 = F::new(5.4655730795145296e-05) * t14903;
    let t14906 = t5931 * t1131 * t1187;
    let t14911 = t2817 * t780 * t1138 * t2820;
    let t14925 = t168 * t2782 * t861;
    let t14932 = t153 * t1125 * t1891;
    (t14904, t14906, t14911, t14925, t14932)
}
