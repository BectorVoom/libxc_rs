//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 601/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk601<F: Float>(t148: F, t2929: F, t1159: F, t242: F, t632: F, t695: F, t1198: F, t1143: F, t458: F, t1155: F, t285: F, t477: F) -> (F, F, F, F, F, F, F) {
    let t4095 = F::new(0.0837628205355044) * t148 * t2929;
    let t4096 = t1159 * t242;
    let t4099 = F::new(0.5025769232130264) * t695 * t632;
    let t4103 = t1198 * t632;
    let t4106 = F::new(0.2512884616065132) * t458 * t1143;
    let t4113 = F::new(0.5025769232130264) * t1155 * t242;
    let t4125 = t1159 * t477 * t285;
    (t4095, t4096, t4099, t4103, t4106, t4113, t4125)
}
