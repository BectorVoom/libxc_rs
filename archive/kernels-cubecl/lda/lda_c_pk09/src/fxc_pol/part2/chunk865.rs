//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 865/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk865<F: Float>(t2263: F, t3104: F, t2247: F, t748: F, t2255: F, t1011: F, t151: F, t164: F, t2271: F, t7706: F, t773: F, t7768: F, t7776: F, t7962: F, t8892: F, t8895: F, t8947: F, t8953: F, t8964: F) -> (F, F, F, F) {
    let t8966 = t3104 * t2263;
    let t8968 = t748 * t2247;
    let t8970 = t3104 * t2255;
    let t8972 = F::cast_from(2.9824072957409817_f64) * t773 * t8892 + F::cast_from(2.9824072957409817_f64) * t773 * t8895 - F::cast_from(0.04115066352984959_f64) * t164 * t8947 + F::cast_from(4.937333717448355_f64) * t2271 * t1011 + F::cast_from(0.04115066352984959_f64) * t164 * t8953 - F::cast_from(1.8805371096875316_f64) * t151 * t7962 - F::cast_from(1.8805371096875316_f64) * t151 * t7768 - F::cast_from(1.8805371096875316_f64) * t151 * t7776 - F::cast_from(1.8805371096875316_f64) * t151 * t7706 - F::cast_from(14.71989892086604_f64) * t8964 + F::cast_from(2.400108951976084_f64) * t8966 - F::cast_from(14.71989892086604_f64) * t8968 - F::cast_from(2.400108951976084_f64) * t8970;
    (t8966, t8968, t8970, t8972)
}
