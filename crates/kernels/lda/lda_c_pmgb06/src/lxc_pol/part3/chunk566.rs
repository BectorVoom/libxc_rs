//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 566/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk566<F: Float>(t1604: F, t486: F, t1636: F, t1588: F, t2947: F, t2951: F, t2955: F, t2959: F, t2964: F, t2968: F, t2973: F, t2976: F, t2978: F, t2982: F, t2986: F, t2990: F, t2995: F) -> (F, F, F, F, F) {
    let t2997 = t486 * t1604 / F::new(5.0);
    let t2998 = t486 * t1636;
    let t2999 = F::new(2.0) / F::new(15.0) * t2998;
    let t3001 = t486 * t1588 / F::new(10.0);
    let t3002 = t2947 - t2951 - t2955 - t2959 - t2964 + t2968 - t2973 - t2976 - t2978 - t2982 - t2986 - t2990 - t2995 + t2997 - t2999 - t3001;
    (t2997, t2998, t2999, t3001, t3002)
}
