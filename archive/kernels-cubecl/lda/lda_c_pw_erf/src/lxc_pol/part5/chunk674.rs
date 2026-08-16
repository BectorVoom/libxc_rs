//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 674/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk674<F: Float>(t2325: F, t2953: F, t2329: F, t939: F, t2849: F, t462: F) -> (F, F, F) {
    let t5982 = t2953 * t2325;
    let t5987 = t939 * t2329;
    let t5992 = -F::cast_from(2.0_f64) * t462 - F::cast_from(6.0_f64) * t2849;
    (t5982, t5987, t5992)
}
