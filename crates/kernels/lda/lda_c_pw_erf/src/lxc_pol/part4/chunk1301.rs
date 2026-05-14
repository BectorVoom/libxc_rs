//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1301/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1301<F: Float>(t16820: F, t16825: F, t16830: F, t16833: F, t16836: F, t16839: F, t16842: F, t16846: F, t16851: F, t16854: F, t16856: F, t16860: F, t16862: F, t16866: F, t16870: F, t16873: F, t16875: F) -> (F,) {
    let t19196 = t16820 - t16825 - t16830 + t16833 + t16836 + t16839 + t16842 - t16846 - t16851 - t16854 - t16856 + t16860 - t16862 + t16866 - t16870 + t16873 + t16875;
    (t19196,)
}
