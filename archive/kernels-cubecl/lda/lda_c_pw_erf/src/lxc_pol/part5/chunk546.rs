//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 546/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk546<F: Float>(t2849: F, t343: F, t35: F, t1128: F, t285: F, t465: F, t281: F, t1184: F, t6: F) -> (F, F, F, F, F, F) {
    let t2850 = F::cast_from(36.0_f64) * t2849;
    let t2851 = t35 * t343;
    let t2852 = F::cast_from(24.0_f64) * t2851;
    let t2863 = t465 * t1128 * t285;
    let t2864 = t281 * t2863;
    let t2869 = t6 * t1184;
    (t2850, t2851, t2852, t2863, t2864, t2869)
}
