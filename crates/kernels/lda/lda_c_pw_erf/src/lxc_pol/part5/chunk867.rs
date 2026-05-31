//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 867/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk867<F: Float>(t5872: F, t5874: F, t7811: F, t7813: F, t7818: F, t7820: F, t7824: F, t7826: F, t7831: F, t7833: F, t7835: F, t7840: F, t7841: F, t7842: F, t7843: F, t7844: F, t7846: F, t7847: F) -> F {
    let t8047 = t7811 - t7813 - t7818 + t7820 + t7824 - t7826 - t7831 + t7833 + t7835 + t7840 - t7841 + t7842 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t5872 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t5874 - t7843 - t7844 - t7846 + t7847;
    t8047
}
