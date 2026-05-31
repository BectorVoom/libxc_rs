//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 395/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk395<F: Float>(t1684: F, t1735: F, t1732: F, t1738: F, t505: F, t452: F, t337: F, t429: F) -> (F, F, F, F, F, F, F) {
    let t1937 = F::cast_from(2.0_f64) * t1684;
    let t1939 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1735;
    let t1941 = t1937 - F::cast_from(2.0_f64) * t1732 + t1939 + F::cast_from(2.0_f64) * t1738;
    let t1942 = F::cast_from(1.0_f64) / t505;
    let t1943 = t1941 * t1942;
    let t1944 = t1943 * t452;
    let t1947 = t337 * t429;
    (t1937, t1939, t1941, t1942, t1943, t1944, t1947)
}
