//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 510/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk510<F: Float>(t2768: F, t2863: F, t2927: F, t2949: F, t417: F, t2138: F, t2440: F, t2445: F, t2696: F, t2701: F, t209: F, t414: F) -> (F, F, F, F) {
    let t2951 = t2768 + t2863 + t2927 + t2949;
    let t2952 = t417 * t2951;
    let t2954 = t2138 / F::cast_from(4.0_f64) + t2440 / F::cast_from(4.0_f64) + t2445 / F::cast_from(8.0_f64) + t2696 / F::cast_from(8.0_f64) + t2701 / F::cast_from(8.0_f64) + t2952 / F::cast_from(8.0_f64);
    let t2956 = t209 / F::cast_from(2.0_f64);
    let t2957 = t414 / F::cast_from(4.0_f64);
    (t2951, t2954, t2956, t2957)
}
