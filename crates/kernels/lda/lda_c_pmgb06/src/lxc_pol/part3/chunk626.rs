//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 626/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk626<F: Float>(t1179: F, t206: F, t4068: F, t2951: F, t2955: F, t2959: F, t2964: F, t2968: F, t2973: F, t2976: F, t2978: F, t2982: F, t2986: F, t2990: F, t2995: F, t2997: F, t2999: F, t3001: F, t3007: F) -> (F, F) {
    let t4070 = 0.001515438175925926 * t206 * t1179 * t4068;
    let t4071 = -t2951 - t2955 - t2959 - t2964 + t2968 - t2973 - t2976 - t2978 - t2982 - t2986 - t2990 - t2995 + t2997 - t2999 - t3001 + t3007 + t4070;
    (t4070, t4071)
}
