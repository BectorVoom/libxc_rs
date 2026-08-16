//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 865/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk865(t2263: f64, t3104: f64, t2247: f64, t748: f64, t2255: f64, t1011: f64, t151: f64, t164: f64, t2271: f64, t7706: f64, t773: f64, t7768: f64, t7776: f64, t7962: f64, t8892: f64, t8895: f64, t8947: f64, t8953: f64, t8964: f64) -> (f64, f64, f64, f64) {
    let t8966 = t3104 * t2263;
    let t8968 = t748 * t2247;
    let t8970 = t3104 * t2255;
    let t8972 = 2.9824072957409817_f64 * t773 * t8892 + 2.9824072957409817_f64 * t773 * t8895 - 0.04115066352984959_f64 * t164 * t8947 + 4.937333717448355_f64 * t2271 * t1011 + 0.04115066352984959_f64 * t164 * t8953 - 1.8805371096875316_f64 * t151 * t7962 - 1.8805371096875316_f64 * t151 * t7768 - 1.8805371096875316_f64 * t151 * t7776 - 1.8805371096875316_f64 * t151 * t7706 - 14.71989892086604_f64 * t8964 + 2.400108951976084_f64 * t8966 - 14.71989892086604_f64 * t8968 - 2.400108951976084_f64 * t8970;
    (t8966, t8968, t8970, t8972)
}
