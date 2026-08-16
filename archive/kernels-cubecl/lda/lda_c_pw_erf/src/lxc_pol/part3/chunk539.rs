//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 539/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk539<F: Float>(t2760: F, t1553: F, t452: F, t405: F, t137: F, t142: F) -> (F, F, F, F) {
    let t2761 = F::cast_from(12.0_f64) * t2760;
    let t2763 = t452 * t1553;
    let t2764 = t405 * t2763;
    let t2765 = t137 * t142;
    (t2761, t2763, t2764, t2765)
}
