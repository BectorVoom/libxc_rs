//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1234/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1234<F: Float>(t8880: F, t8902: F, t8925: F, t14549: F, t14552: F, t14555: F, t14558: F, t14561: F, t8865: F, t8869: F, t8873: F, t8936: F, t9083: F) -> (F, F, F, F) {
    let t14562 = F::new(2.923025) * t8880;
    let t14563 = F::new(5.84605) * t8902;
    let t14564 = F::new(0.48717083333333333) * t8925;
    let t14565 = t8865 - t8869 + t8873 + t14549 + t14552 - t14555 + t14558 + t14561 + t9083 - t14562 + t14563 + t14564 - t8936;
    (t14562, t14563, t14564, t14565)
}
