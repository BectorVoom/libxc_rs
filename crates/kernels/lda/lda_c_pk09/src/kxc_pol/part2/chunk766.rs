//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 766/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk766<F: Float>(t61: F, t825: F, t8944: F, t96: F, t2143: F, t844: F, t873: F, t2251: F, t748: F, t2263: F, t3104: F, t2247: F, t2255: F, t1011: F, t151: F, t164: F, t2271: F, t7706: F, t773: F, t7768: F, t7776: F, t7962: F, t8892: F, t8895: F) -> (F, F, F, F, F) {
    let t8947 = t96 * t61 * t8944 * t825;
    let t8953 = t844 * t873 * t2143;
    let t8964 = t748 * t2251;
    let t8966 = t3104 * t2263;
    let t8968 = t748 * t2247;
    let t8970 = t3104 * t2255;
    let t8972 = 2.9824072957409817 * t773 * t8892 + 2.9824072957409817 * t773 * t8895 - 0.04115066352984959 * t164 * t8947 + 4.937333717448355 * t2271 * t1011 + 0.04115066352984959 * t164 * t8953 - 1.8805371096875316 * t151 * t7962 - 1.8805371096875316 * t151 * t7768 - 1.8805371096875316 * t151 * t7776 - 1.8805371096875316 * t151 * t7706 - 14.71989892086604 * t8964 + 2.400108951976084 * t8966 - 14.71989892086604 * t8968 - 2.400108951976084 * t8970;
    (t8964, t8966, t8968, t8970, t8972)
}
