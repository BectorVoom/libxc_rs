//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 884/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk884<F: Float>(t2874: F, t684: F, t8520: F, t281: F, t285: F, t2853: F, t477: F, t1112: F, t1128: F, t2872: F, t465: F, t2824: F, t6: F) -> (F, F, F, F, F, F) {
    let t8774 = F::cast_from(0.07982822673503073_f64) * t684 * t2874;
    let t8777 = F::new(120.0) * t8520;
    let t8785 = t281 * t2853 * t477 * t285;
    let t8789 = t281 * t1112 * t1128 * t285;
    let t8793 = t281 * t465 * t2872 * t285;
    let t8798 = t6 * t2824;
    (t8774, t8777, t8785, t8789, t8793, t8798)
}
