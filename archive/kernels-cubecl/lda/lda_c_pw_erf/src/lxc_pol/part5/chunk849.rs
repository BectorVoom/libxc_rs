//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 849/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk849<F: Float>(t5343: F, t7017: F, t7019: F, t231: F, t4235: F, t7278: F, t7818: F, t7820: F, t7824: F, t7826: F, t7827: F, t7831: F, t7833: F, t7835: F, t7840: F, t7841: F, t7842: F, t7843: F) -> (F, F, F, F) {
    let t7844 = F::cast_from(8.0_f64) / F::cast_from(135.0_f64) * t5343;
    let t7846 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t7017;
    let t7847 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t7019;
    let t7848 = -t7818 + t7820 + t7824 - t7826 + t4235 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t7827 * t231 - t7831 + t7833 + t7835 + t7840 - t7841 + t7842 - t7843 - t7844 + F::cast_from(4.0_f64) * t7278 - t7846 + t7847;
    (t7844, t7846, t7847, t7848)
}
