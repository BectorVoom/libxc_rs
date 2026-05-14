//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1056/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1056<F: Float>(t21720: F, t21729: F, t21774: F, t21781: F, t21796: F, t21803: F, t21812: F, t21824: F, t11758: F, t19209: F, t19211: F, t19215: F, t19217: F, t19219: F, t19221: F, t19224: F, t19227: F, t19231: F, t19233: F, t19236: F) -> (F, F) {
    let t21827 = t21720 + t21729 + t21774 + t21781 + t21796 + t21803 + t21812 + t21824;
    let t21850 = -t19209 + t19211 - t19215 - t19217 + t19219 + t19221 + t11758 - t19224 - t19227 - t19231 - t19233 + t19236;
    (t21827, t21850)
}
