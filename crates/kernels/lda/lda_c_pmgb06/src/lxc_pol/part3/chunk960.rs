//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 960/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk960<F: Float>(t3098: F, t465: F, t1069: F, t477: F, t760: F, t5083: F, t12514: F, t495: F, t5065: F, t5072: F, t2970: F, t5077: F, t823: F, t3104: F, t2961: F, t5078: F) -> (F, F, F, F, F, F, F) {
    let t13000 = t465 * t3098;
    let t13002 = t760 * t1069 * t477;
    let t13005 = 2.0 / 3.0 * t5083 * t13000 * t13002;
    let t13007 = t5065 * t12514 * t495;
    let t13008 = t13007 * t5072;
    let t13009 = 8.0 / 45.0 * t13008;
    let t13012 = 2.0 / 15.0 * t5077 * t823 * t2970;
    let t13015 = t5083 * t823 * t3104 / 9.0;
    let t13018 = 2.0 / 9.0 * t5083 * t5078 * t2961;
    (t13002, t13005, t13007, t13009, t13012, t13015, t13018)
}
