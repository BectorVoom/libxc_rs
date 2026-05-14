//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1035/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1035<F: Float>(t4615: F, t519: F, t5237: F, t10567: F, t197: F, t3854: F, t4693: F, t571: F, t4671: F, t4794: F, t10527: F, t219: F, t4666: F, t4680: F, t10605: F, t1944: F) -> (F, F, F, F, F, F, F, F) {
    let t14193 = t519 * t5237 * t4615;
    let t14200 = t10567 * t197;
    let t14235 = t571 * t3854 * t4693;
    let t14238 = t571 * t4794 * t4671;
    let t14240 = t10527 * t219;
    let t14242 = t571 * t14240 * t4666;
    let t14245 = t571 * t4794 * t4680;
    let t14255 = t571 * t10605 * t219 * t1944;
    (t14193, t14200, t14235, t14238, t14240, t14242, t14245, t14255)
}
