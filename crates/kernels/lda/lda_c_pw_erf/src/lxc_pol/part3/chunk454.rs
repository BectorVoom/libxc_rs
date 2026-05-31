//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 454/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk454<F: Float>(t1182: F, t1741: F, t116: F, t299: F, t732: F, t310: F, t311: F) -> (F, F, F) {
    let t1742 = t1182 + t1741;
    let t1746 = t732 * t299 * t116;
    let t1750 = F::cast_from(1.0_f64) / t311 / t310;
    (t1742, t1746, t1750)
}
