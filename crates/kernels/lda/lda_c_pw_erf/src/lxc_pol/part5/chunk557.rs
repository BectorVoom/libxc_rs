//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 557/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk557<F: Float>(t2940: F, t2983: F, t2986: F, t400: F, t1055: F, t1059: F, t1010: F, t1022: F, t387: F, t1030: F, t385: F, t1027: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t2987 = t2983 * t2940 * t2986;
    let t2988 = t400 * t2987;
    let t2989 = F::cast_from(1025.3897021007795_f64) * t2988;
    let t2990 = t1059 * t1055;
    let t2991 = F::cast_from(51.94726769812759_f64) * t2990;
    let t2993 = t1010 * t1022 * t387;
    let t2994 = t400 * t2993;
    let t2995 = F::cast_from(3.5089340384731225_f64) * t2994;
    let t2997 = t1030 * t385;
    let t2998 = t1027 * t1022 * t2997;
    (t2987, t2988, t2989, t2990, t2991, t2993, t2994, t2995, t2997, t2998)
}
