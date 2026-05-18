//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 883/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk883<F: Float>(t1063: F, t147: F, t159: F, t285: F, t142: F, t3363: F, t454: F, t1553: F, t1726: F, t405: F, t2863: F, t684: F) -> (F, F, F, F, F) {
    let t8756 = t1063 * t147;
    let t8759 = F::new(0.03831185177913979) * t8756 * t159 * t285;
    let t8761 = t454 * t3363 * t142;
    let t8768 = t405 * t1726 * t1553;
    let t8771 = t684 * t2863;
    (t8756, t8759, t8761, t8768, t8771)
}
