//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 235/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk235<F: Float>(t721: F, t944: F, t62: F, t902: F, t633: F, t131: F, t650: F, t707: F, t125: F, t198: F, t142: F) -> (F, F, F, F, F, F, F, F) {
    let t946 = F::cast_from(1.8805371096875316_f64) * t944 * t721;
    let t947 = t902 * t62;
    let t948 = t947 * t633;
    let t949 = t131 * t948;
    let t952 = t707 * t650;
    let t953 = t131 * t952;
    let t956 = t198 * t125;
    let t957 = t956 * t142;
    (t946, t947, t948, t949, t952, t953, t956, t957)
}
