//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1251/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1251<F: Float>(t2765: F, t2777: F, t756: F, t281: F, t285: F, t4713: F, t477: F, t1128: F, t1904: F, t2872: F, t780: F, t1184: F, t1187: F, t483: F) -> (F, F, F, F, F) {
    let t14876 = t2765 * t756 * t2777;
    let t14891 = t281 * t4713 * t477 * t285;
    let t14895 = t281 * t1904 * t1128 * t285;
    let t14896 = F::new(0.03592270203076383) * t14895;
    let t14899 = t281 * t780 * t2872 * t285;
    let t14903 = t1184 * t1904 * t483 * t1187;
    (t14876, t14891, t14896, t14899, t14903)
}
